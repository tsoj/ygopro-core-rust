//! Generates the card registry: one module per file `src/cards/c<code>.rs`.

use std::fmt::Write;
use std::path::Path;

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("cards");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut cards: Vec<(u32, String)> = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let Some(code) = name.strip_prefix('c').and_then(|x| x.parse::<u32>().ok()) else { continue };
        if path.extension().is_some_and(|e| e == "rs") {
            println!("cargo:rerun-if-changed={}", path.display());
            let has_info = std::fs::read_to_string(&path).unwrap().contains("pub const INFO");
            cards.push((code, if has_info { format!("&{name}::INFO") } else { "&ScriptInfo::DEFAULT".into() }));
        }
    }
    cards.sort();
    let mut out = String::new();
    for (code, _) in &cards {
        writeln!(out, "#[path = {:?}]\npub mod c{code};", dir.join(format!("c{code}.rs")).display().to_string()).unwrap();
    }
    writeln!(out, "/// (script code, `initial_effect`, declarations), by code").unwrap();
    writeln!(out, "pub static ALL: &[(u32, fn(&mut Duel, CardId), &ScriptInfo)] = &[").unwrap();
    for (code, info) in &cards {
        writeln!(out, "    ({code}, c{code}::initial_effect, {info}),").unwrap();
    }
    writeln!(out, "];").unwrap();
    std::fs::write(Path::new(&std::env::var("OUT_DIR").unwrap()).join("cards.rs"), out).unwrap();
}
