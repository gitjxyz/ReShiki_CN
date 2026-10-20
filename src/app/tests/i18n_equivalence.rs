//! Cargo-owned equivalence regression, migrated from V-20261020 (50 baseline cases).
//! The language-changing work runs in a child test process, isolated from parallel app tests.
use reshiki::i18n::{Language, set_language, t, tf};

fn case(name: &str, template: &str, args: &[String], expected_en: &str) -> (usize, usize) {
    // English fallback must reproduce the original format! output exactly.
    set_language(Language::English);
    let en = tf(template, args);
    let en_ok = en == expected_en;
    // zh-CN must keep every argument byte-for-byte inside the translation.
    set_language(Language::ChineseSimplified);
    let zh = tf(template, args);
    let zh_ok = args.iter().all(|a| zh.contains(a.as_str()));
    if !en_ok || !zh_ok {
        println!(
            "FAIL {}\n  en {:?} vs {:?}\n  zh {:?}",
            name, en, expected_en, zh
        );
    }
    (en_ok as usize, zh_ok as usize)
}

fn s(x: &str) -> String {
    x.to_owned()
}

fn verify_templates() {
    let mut pass = (0usize, 0usize);
    let mut total = (0usize, 0usize);
    let mut run = |name: &str, template: &str, args: &[String], expected: &str| {
        let r = case(name, template, args, expected);
        total.0 += 1;
        total.1 += 1;
        pass.0 += r.0;
        pass.1 += r.1;
    };

    // export panel
    run("Export {}…", "Export {}…", &[s("SVG")], "Export SVG…");
    run(
        "Export {} figure",
        "Export {} figure",
        &[s("PDF")],
        "Export PDF figure",
    );
    run(
        "Export {} structure",
        "Export {} structure",
        &[s("MOL")],
        "Export MOL structure",
    );
    run(
        "Figure format: {}",
        "Figure format: {}",
        &[s("PNG")],
        "Figure format: PNG",
    );
    run(
        "Structure format: {}",
        "Structure format: {}",
        &[s("SMILES")],
        "Structure format: SMILES",
    );
    // workspace context / tools
    run(
        "More: {}",
        "More: {}",
        &[s("Reset, Order")],
        "More: Reset, Order",
    );
    run(
        "Straight chain · {}",
        "Straight chain · {} · Drag a regular zigzag toward the pointer",
        &[s("Shift+X")],
        "Straight chain · Shift+X · Drag a regular zigzag toward the pointer",
    );
    run(
        "Atom label · {}",
        "Atom label · {}",
        &[s("C · c")],
        "Atom label · C · c",
    );
    run(
        "keeps ring size",
        "{} · {} keeps ring size",
        &[s("Click or drag to attach"), s("⇧R")],
        "Click or drag to attach · ⇧R keeps ring size",
    );
    // selection summary plural pair
    run("1 atom", "1 atom", &[], "1 atom");
    run("{} atoms", "{} atoms", &[s("3")], "3 atoms");
    run("{} groups", "{} groups", &[s("2")], "2 groups");
    run("1 bond", "1 bond", &[], "1 bond");
    // import
    run("Detected: {}", "Detected: {}", &[s("MOL")], "Detected: MOL");
    run(
        "Insert into the drawing · {}",
        "Insert into the drawing · {}",
        &[s("⌘Enter")],
        "Insert into the drawing · ⌘Enter",
    );
    run(
        "Name below: {}",
        "Name below: {}",
        &[s("aspirin")],
        "Name below: aspirin",
    );
    run(
        "Copy alias {}",
        "Copy alias {}",
        &[s("pubchem")],
        "Copy alias pubchem",
    );
    run(
        "{} atoms · {} bonds",
        "{} atoms · {} bonds",
        &[s("9"), s("8")],
        "9 atoms · 8 bonds",
    );
    // reaction mapping
    run(
        "pairs unmatched",
        "{} pairs · {} reactant / {} product atoms unmatched",
        &[s("12"), s("3"), s("4")],
        "12 pairs · 3 reactant / 4 product atoms unmatched",
    );
    run(
        "Map row",
        "Map {} · {} → {}",
        &[s("4"), s("C #1"), s("C #2")],
        "Map 4 · C #1 → C #2",
    );
    // optimization
    run(
        "pinned",
        "{} pinned · {}",
        &[s("2"), s("1.234 kcal/mol")],
        "2 pinned · 1.234 kcal/mol",
    );
    // assistant
    run(
        "effort default",
        "{}  ·  Default",
        &[s("High")],
        "High  ·  Default",
    );
    run(
        "browser",
        "Could not open the browser. Visit {}",
        &[s("https://reshiki.com")],
        "Could not open the browser. Visit https://reshiki.com",
    );
    // NMR
    run(
        "not predicted",
        "Not predicted: {}",
        &[s("No matching data")],
        "Not predicted: No matching data",
    );
    run(
        "predicted row",
        "Predicted {} ppm; shell {}; {} references; SD {} ppm",
        &[s("1.250"), s("2"), s("7"), s("0.125")],
        "Predicted 1.250 ppm; shell 2; 7 references; SD 0.125 ppm",
    );
    run(
        "select row",
        "Select {} NMR row {}. {}",
        &[s("¹H"), s("C #4 · 1H"), s("ok")],
        "Select ¹H NMR row C #4 · 1H. ok",
    );
    run(
        "summary",
        "{} NMR · {} of {} atoms predicted",
        &[s("¹³C"), s("6"), s("8")],
        "¹³C NMR · 6 of 8 atoms predicted",
    );
    run(
        "calculating",
        "Calculating {} shifts",
        &[s("¹H")],
        "Calculating ¹H shifts",
    );
    run(
        "marker",
        "Predicted shift marker. {}.{}",
        &[s("C #1"), s("")],
        "Predicted shift marker. C #1.",
    );
    run(
        "additional sites",
        "; {} additional sites. Full predictions are in the results table",
        &[s("5")],
        "; 5 additional sites. Full predictions are in the results table",
    );
    run(
        "shown at abbreviation",
        ". Shown at abbreviation {} anchored on atom #{}",
        &[s("OMe"), s("3")],
        ". Shown at abbreviation OMe anchored on atom #3",
    );
    run(
        "shell details",
        "Shell {} · {} references · SD {} ppm",
        &[s("2"), s("7"), s("0.125")],
        "Shell 2 · 7 references · SD 0.125 ppm",
    );
    // reference panel
    run(
        "arc sweep",
        "Arc sweep {} degrees",
        &[s("180")],
        "Arc sweep 180 degrees",
    );
    run(
        "arc field",
        "Arc {} (degrees)",
        &[s("Sweep")],
        "Arc Sweep (degrees)",
    );
    run(
        "enter finite",
        "Enter a finite {}",
        &[s("length")],
        "Enter a finite length",
    );
    run(
        "shape edge",
        "Shape {} · edge {}",
        &[s("2"), s("1")],
        "Shape 2 · edge 1",
    );
    run(
        "stretch line (NBSP)",
        "Length {}\u{a0}pt · {}\u{a0}({}) stays fixed",
        &[s("1.42"), s("C"), s("start")],
        "Length 1.42\u{a0}pt · C\u{a0}(start) stays fixed",
    );
    // updates / files / clipboard
    run(
        "update ready",
        "Update ready. {}",
        &[s("Save your drawings.")],
        "Update ready. Save your drawings.",
    );
    run(
        "nightly switch",
        "Switch from Nightly to stable ReShiki {}.",
        &[s("0.12.2")],
        "Switch from Nightly to stable ReShiki 0.12.2.",
    );
    run("version", "Version {}", &[s("0.12.1")], "Version 0.12.1");
    run(
        "prefs error",
        "Could not save update preferences: {}",
        &[s("disk full")],
        "Could not save update preferences: disk full",
    );
    run(
        "save question",
        "Do you want to save the changes you made to “{}”?",
        &[s("draw.rsk")],
        "Do you want to save the changes you made to “draw.rsk”?",
    );
    run(
        "preparing copy",
        "Preparing {} for the clipboard…",
        &[s("MOL")],
        "Preparing MOL for the clipboard…",
    );
    run(
        "copy failed",
        "Could not copy {}: {} · Clipboard unchanged",
        &[s("MOL"), s("timeout")],
        "Could not copy MOL: timeout · Clipboard unchanged",
    );
    run(
        "copied as",
        "{} copied as {}",
        &[s("Drawing"), s("SMILES")],
        "Drawing copied as SMILES",
    );
    // language-switch template
    run(
        "language save error",
        "Language changed, but could not save preference: {}",
        &[s("locked")],
        "Language changed, but could not save preference: locked",
    );

    // empty/unicode/brace stress on three representative templates
    run("empty arg", "Detected: {}", &[s("")], "Detected: ");
    run(
        "cjk arg",
        "More: {}",
        &[s("重置、排序")],
        "More: 重置、排序",
    );
    run("braces arg", "Export {}…", &[s("a{b}c")], "Export a{b}c…");
    run(
        "newline arg",
        "Name below: {}",
        &[s("a\nb")],
        "Name below: a\nb",
    );

    run(
        "{} · {}",
        "{} · {}",
        &[s("Brackets"), s("Both sides")],
        "Brackets · Both sides",
    );
    run(
        "{} {}",
        "{} {}",
        &[s("Outline"), s("Rectangle")],
        "Outline Rectangle",
    );
    run(
        "{} ({})",
        "{} ({})",
        &[s("Bond length"), s("mm")],
        "Bond length (mm)",
    );
    run(
        "{}: {}",
        "{}: {}",
        &[s("Label size (pt)"), s("Finish the number.")],
        "Label size (pt): Finish the number.",
    );
    run(
        "{} must be between {} and {} {}.",
        "{} must be between {} and {} {}.",
        &[s("Label size (pt)"), s("4"), s("144"), s("pt")],
        "Label size (pt) must be between 4 and 144 pt.",
    );
    run(
        "This draft still uses {}. Finish the dimension and press Enter to use {}.",
        "This draft still uses {}. Finish the dimension and press Enter to use {}.",
        &[s("pt"), s("mm")],
        "This draft still uses pt. Finish the dimension and press Enter to use mm.",
    );
    run(
        "{} applied · Undo restores the previous style and layout",
        "{} applied · Undo restores the previous style and layout",
        &[s("Nature")],
        "Nature applied · Undo restores the previous style and layout",
    );
    run(
        "Could not open publisher instructions: {}",
        "Could not open publisher instructions: {}",
        &[s("Error {a}\n中文")],
        "Could not open publisher instructions: Error {a}\n中文",
    );
    run(
        "Units unchanged. {}",
        "Units unchanged. {}",
        &[s("Finish {1}")],
        "Units unchanged. Finish {1}",
    );
    run(
        "Could not save dimension unit preference: {}",
        "Could not save dimension unit preference: {}",
        &[s("disk full")],
        "Could not save dimension unit preference: disk full",
    );
    run(
        "Donor: {} (atom {})",
        "Donor: {} (atom {})",
        &[s("N"), s("17")],
        "Donor: N (atom 17)",
    );
    run(
        "{} {} ring interior(s)",
        "{} {} ring interior(s)",
        &[s("Colored"), s("2")],
        "Colored 2 ring interior(s)",
    );
    run(
        "Color applied to {}",
        "Color applied to {}",
        &[s("all selected")],
        "Color applied to all selected",
    );
    run(
        "No color change · Apply to {}",
        "No color change · Apply to {}",
        &[s("all selected")],
        "No color change · Apply to all selected",
    );
    run(
        "Custom {} · stays exact",
        "Custom {} · stays exact",
        &[s("#1F4E79")],
        "Custom #1F4E79 · stays exact",
    );
    run(
        "Default hues for {}.",
        "Default hues for {}.",
        &[s("Theme {甲}")],
        "Default hues for Theme {甲}.",
    );
    run(
        "Saved as “{} · custom hues”. The built-in theme stays unchanged.",
        "Saved as “{} · custom hues”. The built-in theme stays unchanged.",
        &[s("Theme {甲}")],
        "Saved as “Theme {甲} · custom hues”. The built-in theme stays unchanged.",
    );
    run(
        "Saved with “{}” in this drawing.",
        "Saved with “{}” in this drawing.",
        &[s("Theme {甲}")],
        "Saved with “Theme {甲}” in this drawing.",
    );
    run(
        "parameter bytes",
        "{} · {}",
        &[s("参数0 {literal} \n🧪  0"), s("参数1 {literal} \n🧪  1")],
        "参数0 {literal} \n🧪  0 · 参数1 {literal} \n🧪  1",
    );
    run(
        "parameter bytes",
        "{} {}",
        &[s("参数0 {literal} \n🧪  0"), s("参数1 {literal} \n🧪  1")],
        "参数0 {literal} \n🧪  0 参数1 {literal} \n🧪  1",
    );
    run(
        "parameter bytes",
        "{} ({})",
        &[s("参数0 {literal} \n🧪  0"), s("参数1 {literal} \n🧪  1")],
        "参数0 {literal} \n🧪  0 (参数1 {literal} \n🧪  1)",
    );
    run(
        "parameter bytes",
        "{}: {}",
        &[s("参数0 {literal} \n🧪  0"), s("参数1 {literal} \n🧪  1")],
        "参数0 {literal} \n🧪  0: 参数1 {literal} \n🧪  1",
    );
    run(
        "parameter bytes",
        "{} must be between {} and {} {}.",
        &[
            s("参数0 {literal} \n🧪  0"),
            s("参数1 {literal} \n🧪  1"),
            s("参数2 {literal} \n🧪  2"),
            s("参数3 {literal} \n🧪  3"),
        ],
        "参数0 {literal} \n🧪  0 must be between 参数1 {literal} \n🧪  1 and 参数2 {literal} \n🧪  2 参数3 {literal} \n🧪  3.",
    );
    run(
        "parameter bytes",
        "This draft still uses {}. Finish the dimension and press Enter to use {}.",
        &[s("参数0 {literal} \n🧪  0"), s("参数1 {literal} \n🧪  1")],
        "This draft still uses 参数0 {literal} \n🧪  0. Finish the dimension and press Enter to use 参数1 {literal} \n🧪  1.",
    );
    run(
        "parameter bytes",
        "{} applied · Undo restores the previous style and layout",
        &[s("参数0 {literal} \n🧪  0")],
        "参数0 {literal} \n🧪  0 applied · Undo restores the previous style and layout",
    );
    run(
        "parameter bytes",
        "Could not open publisher instructions: {}",
        &[s("参数0 {literal} \n🧪  0")],
        "Could not open publisher instructions: 参数0 {literal} \n🧪  0",
    );
    run(
        "parameter bytes",
        "Units unchanged. {}",
        &[s("参数0 {literal} \n🧪  0")],
        "Units unchanged. 参数0 {literal} \n🧪  0",
    );
    run(
        "parameter bytes",
        "Could not save dimension unit preference: {}",
        &[s("参数0 {literal} \n🧪  0")],
        "Could not save dimension unit preference: 参数0 {literal} \n🧪  0",
    );
    run(
        "parameter bytes",
        "Donor: {} (atom {})",
        &[s("参数0 {literal} \n🧪  0"), s("参数1 {literal} \n🧪  1")],
        "Donor: 参数0 {literal} \n🧪  0 (atom 参数1 {literal} \n🧪  1)",
    );
    run(
        "parameter bytes",
        "{} {} ring interior(s)",
        &[s("参数0 {literal} \n🧪  0"), s("参数1 {literal} \n🧪  1")],
        "参数0 {literal} \n🧪  0 参数1 {literal} \n🧪  1 ring interior(s)",
    );
    run(
        "parameter bytes",
        "Color applied to {}",
        &[s("参数0 {literal} \n🧪  0")],
        "Color applied to 参数0 {literal} \n🧪  0",
    );
    run(
        "parameter bytes",
        "No color change · Apply to {}",
        &[s("参数0 {literal} \n🧪  0")],
        "No color change · Apply to 参数0 {literal} \n🧪  0",
    );
    run(
        "parameter bytes",
        "Custom {} · stays exact",
        &[s("参数0 {literal} \n🧪  0")],
        "Custom 参数0 {literal} \n🧪  0 · stays exact",
    );
    run(
        "parameter bytes",
        "Default hues for {}.",
        &[s("参数0 {literal} \n🧪  0")],
        "Default hues for 参数0 {literal} \n🧪  0.",
    );
    run(
        "parameter bytes",
        "Saved as “{} · custom hues”. The built-in theme stays unchanged.",
        &[s("参数0 {literal} \n🧪  0")],
        "Saved as “参数0 {literal} \n🧪  0 · custom hues”. The built-in theme stays unchanged.",
    );
    run(
        "parameter bytes",
        "Saved with “{}” in this drawing.",
        &[s("参数0 {literal} \n🧪  0")],
        "Saved with “参数0 {literal} \n🧪  0” in this drawing.",
    );
    set_language(Language::English);
    // Miss-path byte identity for plain keys used verbatim.
    assert_eq!(t("Exporting…"), "Exporting…");
    assert_eq!(t("ReShiki"), "ReShiki");

    println!(
        "equivalence: {} / {} en-ok, {} / {} zh-args-ok",
        pass.0, total.0, pass.1, total.1
    );
    assert_eq!(pass, total, "equivalence failures above");
    println!("ALL PASS");
}

#[test]
fn localized_templates_and_labels() {
    const NAME: &str = "app::tests::i18n_equivalence::localized_templates_and_labels";
    if std::env::var("RESHIKI_I18N_EQUIVALENCE_CHILD").as_deref() != Ok("1") {
        let result = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .args(["--exact", NAME, "--nocapture"])
            .env("RESHIKI_I18N_EQUIVALENCE_CHILD", "1")
            .output()
            .expect("isolated equivalence test");
        println!("{}", String::from_utf8_lossy(&result.stdout));
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        return;
    }
    verify_templates();
    verify_display_routes();
}

fn verify_display_routes() {
    use super::super::{
        document_styles::Field,
        inspector::LabeledBondPreset,
        palettes::{Family, graphic_options},
        typography::ColorScope,
    };
    use reshiki::{bonds::BondPreset, style::units::Unit};
    set_language(Language::English);
    for preset in BondPreset::ALL {
        assert_eq!(LabeledBondPreset(preset).to_string(), preset.to_string());
        assert_eq!(LabeledBondPreset(preset).0, preset);
    }
    let scopes = [
        "All selected",
        "Text",
        "Bonds",
        "Ring interiors",
        "Highlights",
    ];
    for (scope, expected) in ColorScope::ALL.into_iter().zip(scopes) {
        assert_eq!(scope.to_string(), expected);
    }
    let families = [Family::Rectangles, Family::Ellipses, Family::Brackets];
    let english: Vec<_> = families
        .iter()
        .map(|&family| graphic_options(family))
        .collect();
    assert_eq!(
        english[0]
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        [
            "Outline Rectangle",
            "Dashed Rectangle",
            "Filled Rectangle",
            "Outline Rounded rectangle",
            "Dashed Rounded rectangle",
            "Filled Rounded rectangle",
            "Square"
        ]
    );
    assert_eq!(
        english[1]
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        [
            "Outline Ellipse",
            "Dashed Ellipse",
            "Filled Ellipse",
            "Circle"
        ]
    );
    assert_eq!(
        english[2]
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        [
            "Brackets · Both sides",
            "Brackets · Left only",
            "Brackets · Right only",
            "Parentheses · Both sides",
            "Parentheses · Left only",
            "Parentheses · Right only",
            "Braces · Both sides",
            "Braces · Left only",
            "Braces · Right only"
        ]
    );
    set_language(Language::ChineseSimplified);
    let chinese = |text: &str| text.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c));
    for preset in BondPreset::ALL {
        let value = LabeledBondPreset(preset).to_string();
        assert_eq!(value, t(preset.name()));
        assert!(chinese(&value), "{value}");
    }
    for scope in ColorScope::ALL {
        assert!(chinese(&scope.to_string()));
    }
    for (index, family) in families.into_iter().enumerate() {
        let zh = graphic_options(family);
        for ((en_label, en_option), (zh_label, zh_option)) in english[index].iter().zip(&zh) {
            assert_eq!(
                en_option, zh_option,
                "display must not change selection data"
            );
            assert_ne!(en_label, zh_label);
            assert!(chinese(zh_label), "{zh_label}");
        }
        assert_eq!(english[index].len(), zh.len());
    }
    for unit in Unit::ALL {
        assert_eq!(Field::Bond.label(unit), format!("键长（{unit}）"));
    }
    // Exercise real status consumers as well as their templates.
    let (mut app, _) = super::super::App::new();
    // App construction reloads the saved preference. Select zh explicitly
    // after that boundary so these assertions always exercise Chinese.
    set_language(Language::ChineseSimplified);
    assert_eq!(reshiki::i18n::language(), Language::ChineseSimplified);
    app.apply_ring_color(None);
    assert_eq!(
        app.status,
        t("Select every atom of a ring to change its interior color")
    );
    app.apply_highlight_color(None);
    assert_eq!(
        app.status,
        t("Select atoms or bonds to change their highlights")
    );
    app.arrow_action(super::super::arrows::Action::AttachTargets(true));
    assert_eq!(
        app.status,
        t("Attached arrows · Click a source, then a destination")
    );
    set_language(Language::English);
    println!(
        "Display routes: 17 bond presets + 5 scopes + 20 shape options; selection data unchanged; real status routes PASS"
    );
}
