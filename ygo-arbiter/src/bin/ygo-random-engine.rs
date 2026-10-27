//! The reference engine (PROTOCOL.md): random valid responses (`random::RandomAgent`).
//!
//!   ygo-random-engine
//!
//! Option Seed: game n of the process plays with the seed Seed + n.

use serde_json::Map;
use ygo_arbiter::engine::serve;
use ygo_arbiter::game::init;
use ygo_arbiter::protocol::{EngineOption, Identity};
use ygo_arbiter::random::RandomAgent;

fn main() {
    if let Err(e) = init(None) {
        eprintln!("ygo-random-engine: {e}");
        std::process::exit(1);
    }
    let id = Identity { name: "ygo-random-engine".into(), more: Map::from_iter([("version".into(), env!("CARGO_PKG_VERSION").into())]) };
    let options = vec![EngineOption::Spin { name: "Seed".into(), default: 0, min: 0, max: i64::MAX / 2 }];
    let mut n = 0;
    let factory = |values: &Map<String, serde_json::Value>| {
        let seed = values["Seed"].as_i64().unwrap() + n;
        n += 1;
        Ok(Box::new(RandomAgent::new(seed)) as Box<dyn ygo_arbiter::game::Agent>)
    };
    if let Err(e) = serve(id, options, factory) {
        eprintln!("ygo-random-engine: {e}");
        std::process::exit(1);
    }
}
