//! Runtime interface-language selection with English fallback.
//!
//! Every translatable interface string passes through [`t`], whose key is the
//! English source text. When the active language has no entry for a key, the
//! key itself is returned, so untranslated strings always render English and
//! the English build is byte-for-byte unchanged.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use serde::{Deserialize, Serialize};

/// Flat English-source → translation map embedded from `locales/zh-CN.json`.
static ZH_CN_JSON: &str = include_str!("../locales/zh-CN.json");

const PREFERENCE_FILE: &str = "language-preference.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "zh-CN")]
    ChineseSimplified,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.label())
    }
}

impl Language {
    pub const ALL: [Self; 2] = [Self::English, Self::ChineseSimplified];

    /// Each language names itself in the View panel.
    pub fn label(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::ChineseSimplified => "简体中文",
        }
    }

    /// Only Simplified-Chinese system locales select the zh-CN dictionary;
    /// every other locale keeps English until a matching dictionary exists.
    fn from_system_tag(tag: &str) -> Option<Self> {
        let lower = tag.replace('_', "-").to_ascii_lowercase();
        if lower.starts_with("zh-cn")
            || lower.starts_with("zh-sg")
            || lower.starts_with("zh-hans")
        {
            Some(Self::ChineseSimplified)
        } else {
            None
        }
    }

    fn dictionary(self) -> Option<&'static HashMap<&'static str, &'static str>> {
        match self {
            Self::English => None,
            Self::ChineseSimplified => Some(ZH_CN.get_or_init(|| {
                serde_json::from_str(ZH_CN_JSON).unwrap_or_default()
            })),
        }
    }
}

static LANGUAGE: RwLock<Language> = RwLock::new(Language::English);

static ZH_CN: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

/// Translate `key` for the active language; the key is the English text.
pub fn t(key: &str) -> &str {
    let language = *LANGUAGE
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match language.dictionary() {
        Some(dictionary) => dictionary.get(key).copied().unwrap_or(key),
        None => key,
    }
}

/// The active interface language.
pub fn language() -> Language {
    *LANGUAGE
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Switch the interface language at runtime.
pub fn set_language(language: Language) {
    *LANGUAGE
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = language;
}

/// Resolve the initial language: a saved preference first, then the system
/// UI locale, then English.
pub fn initialize() -> Language {
    let saved = crate::compatibility::data_directory()
        .ok()
        .and_then(|directory| std::fs::read(directory.join(PREFERENCE_FILE)).ok())
        .and_then(|bytes| serde_json::from_slice(&bytes).ok());
    let detected = sys_locale::get_locale()
        .as_deref()
        .and_then(Language::from_system_tag);
    let language = saved.or(detected).unwrap_or_default();
    set_language(language);
    language
}

/// Persist the language choice next to the other preference files.
pub async fn save_language(language: Language) -> Result<(), String> {
    tokio::task::spawn_blocking(move || persist(language))
        .await
        .map_err(|error| error.to_string())?
}

fn persist(language: Language) -> Result<(), String> {
    let directory = crate::compatibility::data_directory()?;
    std::fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let bytes = serde_json::to_vec(&language).map_err(|error| error.to_string())?;
    crate::storage::write_atomic(&directory.join(PREFERENCE_FILE), &bytes)
}
