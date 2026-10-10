//! Card data from the BabelCDB sqlite files (the same files and order as ygoai/carddb.py).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::constants::TYPE_LINK;
use crate::types::{CardData, SetCodes};

#[derive(Clone, Default)]
pub struct CardDb {
    pub cards: HashMap<u32, CardData>,
    pub names: HashMap<u32, String>,
}

/// Later files override earlier ones.
const DEFAULT_CDBS: &[&str] = &["cards-unofficial.cdb", "prerelease-*.cdb", "release-*.cdb", "cards.cdb"];
const EXCLUDED_CDB_PARTS: &[&str] = &["rush", "skills", "goat"];

fn glob(dir: &Path, pattern: &str) -> Vec<PathBuf> {
    let (prefix, suffix) = match pattern.split_once('*') {
        Some((p, s)) => (p.to_string(), s.to_string()),
        None => (pattern.to_string(), String::new()),
    };
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if pattern.contains('*') {
                        name.starts_with(&prefix) && name.ends_with(&suffix) && name.len() >= prefix.len() + suffix.len()
                    } else {
                        name == pattern
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

impl CardDb {
    pub fn load(cdb_dir: &Path) -> rusqlite::Result<CardDb> {
        let mut files: Vec<PathBuf> = vec![];
        for pattern in DEFAULT_CDBS {
            for f in glob(cdb_dir, pattern) {
                let name = f.file_name().unwrap().to_string_lossy().to_string();
                if EXCLUDED_CDB_PARTS.iter().any(|p| name.contains(p)) {
                    continue;
                }
                if !files.contains(&f) {
                    files.push(f);
                }
            }
        }
        let mut db = CardDb::default();
        for f in &files {
            db.load_file(f)?;
        }
        Ok(db)
    }

    fn load_file(&mut self, path: &Path) -> rusqlite::Result<()> {
        let con = rusqlite::Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        let mut stmt = con.prepare(
            "SELECT d.id, d.alias, d.setcode, d.type, d.atk, d.def, d.level, d.race, d.attribute, t.name \
             FROM datas d LEFT JOIN texts t ON d.id = t.id",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, Option<i64>>(2)?.unwrap_or(0),
                r.get::<_, i64>(3)?,
                r.get::<_, i64>(4)?,
                r.get::<_, i64>(5)?,
                r.get::<_, i64>(6)?,
                r.get::<_, i64>(7)?,
                r.get::<_, i64>(8)?,
                r.get::<_, Option<String>>(9)?.unwrap_or_default(),
            ))
        })?;
        for row in rows {
            let (code, alias, setcode, typ, atk, mut def, level, race, attribute, name) = row?;
            let typ = typ as u32;
            let mut link_marker = 0;
            if typ & TYPE_LINK != 0 {
                link_marker = def as u32;
                def = 0;
            }
            let mut setcodes = SetCodes::default();
            for i in 0..4 {
                let sc = ((setcode as u64 >> (16 * i)) & 0xffff) as u16;
                if sc != 0 {
                    setcodes.insert(sc);
                }
            }
            let code = code as u32;
            self.cards.insert(
                code,
                CardData {
                    code,
                    alias: alias as u32,
                    setcodes,
                    type_: typ,
                    level: (level & 0xff) as u32,
                    attribute: attribute as u32,
                    race: race as u64,
                    attack: atk as i32,
                    defense: def as i32,
                    lscale: ((level >> 24) & 0xff) as u32,
                    rscale: ((level >> 16) & 0xff) as u32,
                    link_marker,
                },
            );
            self.names.insert(code, name);
        }
        Ok(())
    }

    /// `duel::read_card`: unknown codes give empty data (with the code).
    pub fn get(&self, code: u32) -> CardData {
        self.cards.get(&code).copied().unwrap_or(CardData { code, ..Default::default() })
    }
}
