//! The card scripts (`external/CardScripts`) in Rust: the global scripts (`utility.lua`, the
//! `proc_*.lua` procedures, the rules they add), and one module per card.
//!
//! Scripts call the API through `lua` (the core's API with the changes the global Lua scripts
//! make to it), write card functions with `prelude`, and use the constants of `constant.lua`.

pub mod aux;
pub mod cards;
pub mod chain;
pub mod constants;
pub mod cost;
pub mod global;
pub mod lua;
pub mod luatable;
pub mod prelude;
pub mod procs;

/// Namespaces of `Duel::script_var` keys `(namespace, key)`: all script state lives in the duel,
/// so that it is copied with it. A card script's own state (`s.x`, `s[0]`, ...) uses its card
/// code as the namespace (see `prelude::s_var`); codes are larger than these.
pub mod ns {
    /// The global scripts' state (`global::vars`)
    pub const GLOBAL: u32 = 0;
    /// `aux.*` state (`aux::vars`)
    pub const AUX: u32 = 1;
    /// `aux.GlobalCheck`: `s.global_check` by card code
    pub const GLOBAL_CHECK: u32 = 2;
    /// `e:GetChainData()` (`chain.rs`)
    pub const CHAIN_DATA: u32 = 3;
    /// Mutable upvalues of library closures (`cell`)
    pub const CELLS: u32 = 4;
    /// Values the library keeps per card (`card_var`)
    pub const CARD_VARS: u32 = 5;
    /// Lua global variables that scripts share (`_replace_count`), keyed by a hash of the name
    pub const LUA_GLOBALS: u32 = 6;
    /// Keys from here on in a card's namespace are fields the library sets on the card's script
    /// table (`class`); a card's own `s_var` keys stay below.
    pub const CLASS_FIELDS: u64 = 1 << 40;
}

/// The mutable upvalues of library closures (a Lua closure that reassigns a captured variable,
/// e.g. `chkf` in `Fusion.SummonEffTG`): cells in the duel, so they are copied with it. A cell is
/// made when the closure is (in `initial_effect`, so in the same order in every copy).
pub mod cell {
    use crate::ns::CELLS;
    use ygo_core::duel::Duel;
    use ygo_core::types::Value;
    pub fn new(d: &mut Duel) -> u64 {
        let id = d.script_var((CELLS, 0)).int() + 1;
        d.set_script_var((CELLS, 0), Value::Int(id));
        id as u64
    }
    pub fn get(d: &Duel, id: u64) -> Value {
        d.script_var((CELLS, id))
    }
    /// Sets an integer cell (no change: no write, so copies keep sharing the script state).
    pub fn set_int(d: &mut Duel, id: u64, v: i64) {
        if !matches!(get(d, id), Value::Int(x) if x == v) {
            d.set_script_var((CELLS, id), Value::Int(v));
        }
    }
}

/// Values the library keeps per card (fields of a card's own table in Lua, e.g.
/// `mt.ritual_matching_function[c]`): `(ns::CARD_VARS, card << 8 | field)`.
pub mod card_var {
    use crate::ns::CARD_VARS;
    use ygo_core::duel::Duel;
    use ygo_core::types::{CardId, Value};
    /// `ritual_matching_function[c]` (`Ritual.CreateProc`)
    pub const RITUAL_MATCHING_FUNCTION: u64 = 1;
    /// `mt.has_malefic_unique[c]` (`aux.MaleficUniqueFilter(c)`; read by "Malefic Territory")
    pub const HAS_MALEFIC_UNIQUE: u64 = 2;
    pub fn get(d: &Duel, c: CardId, field: u64) -> Value {
        d.script_var((CARD_VARS, ((c.0 as u64) << 8) | field))
    }
    pub fn set(d: &mut Duel, c: CardId, field: u64, v: Value) {
        d.set_script_var((CARD_VARS, ((c.0 as u64) << 8) | field), v);
    }
}

/// Fields the library sets on a card's script table at run time (e.g. `Xyz.AddProcedure` sets
/// `s.minxyzct`): `Duel::script_var((script code, ns::CLASS_FIELDS + field))`.
pub mod class {
    use crate::ns::CLASS_FIELDS;
    /// `minxyzct`, `maxxyzct` (`Xyz.AddProcedure`)
    pub const MIN_XYZ_CT: u64 = CLASS_FIELDS + 1;
    pub const MAX_XYZ_CT: u64 = CLASS_FIELDS + 2;
    /// `material` (the codes in a Fusion procedure: overrides `s.material`), `material_count`,
    /// `min_material_count`, `max_material_count` (`Fusion.AddProcMix`, ...)
    pub const MATERIAL: u64 = CLASS_FIELDS + 3;
    pub const MATERIAL_COUNT: u64 = CLASS_FIELDS + 4;
    pub const MIN_MATERIAL_COUNT: u64 = CLASS_FIELDS + 5;
    pub const MAX_MATERIAL_COUNT: u64 = CLASS_FIELDS + 6;
    /// `old_union` (`aux.AddUnionProcedure` with the old rules)
    pub const OLD_UNION: u64 = CLASS_FIELDS + 7;
    /// `min_tribute_req`, `max_tribute_req` (`aux.AddNormalSummonProcedure`/`AddNormalSetProcedure`)
    pub const MIN_TRIBUTE_REQ: u64 = CLASS_FIELDS + 8;
    pub const MAX_TRIBUTE_REQ: u64 = CLASS_FIELDS + 9;
    /// `fit_monster` (`Ritual.AddProcCode`; overrides `s.fit_monster`)
    pub const FIT_MONSTER: u64 = CLASS_FIELDS + 10;
    /// `announce_filter`: the `Duel.AnnounceCard` opcodes a script last declared with (set at run
    /// time by the declaring cards; read by "Engraver of the Mark")
    pub const ANNOUNCE_FILTER: u64 = CLASS_FIELDS + 11;
    /// `s.announce_filter = {...}` of the script with code `code`
    pub fn set_announce_filter(d: &mut ygo_core::duel::Duel, code: u32, filter: &[u64]) {
        d.set_script_var((code, ANNOUNCE_FILTER), ygo_core::types::Value::Multi(filter.iter().map(|&x| ygo_core::types::Value::Int(x as i64)).collect()));
    }
    /// `c.announce_filter` (None: nil)
    pub fn announce_filter(d: &ygo_core::duel::Duel, c: ygo_core::types::CardId) -> Option<Vec<u64>> {
        match get(d, c, ANNOUNCE_FILTER) {
            ygo_core::types::Value::Multi(m) => Some(m.iter().map(|v| v.int() as u64).collect()),
            _ => None,
        }
    }
    /// `c.fit_monster`
    pub fn fit_monster(d: &ygo_core::duel::Duel, c: ygo_core::types::CardId) -> Vec<u32> {
        match get(d, c, FIT_MONSTER) {
            ygo_core::types::Value::Multi(m) => m.iter().map(|v| v.int() as u32).collect(),
            _ => crate::card_info(d, c).fit_monster.to_vec(),
        }
    }
    /// `c.material`: the procedure's list if it set one, else the script's `s.material`
    pub fn material(d: &ygo_core::duel::Duel, c: ygo_core::types::CardId) -> Vec<u32> {
        match get(d, c, MATERIAL) {
            ygo_core::types::Value::Multi(m) => m.iter().map(|v| v.int() as u32).collect(),
            _ => crate::card_info(d, c).material.to_vec(),
        }
    }
    /// The class field `f` of the script `c` uses.
    pub fn get(d: &ygo_core::duel::Duel, c: ygo_core::types::CardId, f: u64) -> ygo_core::types::Value {
        d.script_var((d.card(c).script_code, f))
    }
    pub fn set(d: &mut ygo_core::duel::Duel, c: ygo_core::types::CardId, f: u64, v: ygo_core::types::Value) {
        let code = d.card(c).script_code;
        d.set_script_var((code, f), v);
    }
}

use ygo_core::script::Scripts;

/// All card scripts (code -> initial_effect).
pub fn scripts() -> Scripts {
    let mut s = Scripts::default();
    for &(code, f, _) in cards::ALL {
        s.insert(code, f);
    }
    s
}

/// What loading `constant.lua`, `utility.lua` (with the scripts it loads) and the project's
/// overrides does to a new duel: the global rule effects.
pub fn load_global_scripts(d: &mut ygo_core::duel::Duel) {
    global::load(d);
}

/// What a card's script declares besides its effects (fields of the Lua card table, e.g.
/// `s.listed_names={...}`).
#[derive(Clone, Copy, Debug)]
pub struct ScriptInfo {
    pub maximum_side: Option<global::MaximumSide>,
    /// `s.self_equip_trap`
    pub self_equip_trap: bool,
    /// The scripts the script loads (`Duel.LoadCardScript`): their metatables exist in the duel
    /// (`current_info`) as if a card with that code took part
    pub loads: &'static [u32],
    /// `s.listed_names`
    pub listed_names: &'static [u32],
    /// `s.fit_monster`
    pub fit_monster: &'static [u32],
    /// `s.listed_card_types`
    pub listed_card_types: &'static [u32],
    /// `s.listed_series`
    pub listed_series: &'static [u32],
    /// `s.listed_cards`
    pub listed_cards: &'static [u32],
    /// `s.material`
    pub material: &'static [u32],
    /// `s.material_setcode`
    pub material_setcode: &'static [u32],
    /// `s.min_tribute_req`, `s.max_tribute_req`
    pub tribute_req: Option<(u32, u32)>,
    /// `s.counter_list`
    pub counter_list: &'static [u32],
    /// `s.counter_place_list`
    pub counter_place_list: &'static [u32],
    /// `s.dark_calling` (also set by `c:AddMustBeSpecialSummonedByDarkFusion()`)
    pub dark_calling: bool,
    /// `s.pendulum_level`: the Level a Pendulum Summon checks (monsters without a Level)
    pub pendulum_level: Option<u32>,
    /// `s.mat_filter(c, tp)`: the Ritual Monster's own material restriction
    pub mat_filter: Option<fn(&mut ygo_core::duel::Duel, ygo_core::types::CardId, u8) -> bool>,
    /// `s.ritual_custom_check(e, tp, g, c)`: (the materials are valid, stop adding materials)
    pub ritual_custom_check: Option<fn(&mut ygo_core::duel::Duel, ygo_core::types::EffectId, u8, &ygo_core::types::CardSet, ygo_core::types::CardId) -> (bool, bool)>,
    /// `s.roll_dice` (read by cards that support dice cards)
    pub roll_dice: bool,
    /// `s.toss_coin` (read by cards that support coin cards)
    pub toss_coin: bool,
    /// `s.LVnum`, `s.LVset`: the Level and set code of a LV monster (e.g. read by Level Up!)
    pub lv: Option<(u32, u32)>,
    /// `s.xyz_number` (the "Number" of a Number monster)
    pub xyz_number: Option<u32>,
    /// `s.synchro_nt_required` (how many non-Tuners the Synchro needs, read by other cards)
    pub synchro_nt_required: Option<u32>,
    /// `s.synchro_tuner_required`
    pub synchro_tuner_required: Option<u32>,
    /// `s.miracle_synchro_fusion`
    pub miracle_synchro_fusion: bool,
    /// `s.assault_mode`: the code of the monster this "/Assault Mode" card comes from
    pub assault_mode: Option<u32>,
    /// `s.self_tuner`
    pub self_tuner: bool,
    /// `s.material_trap`: the Trap Card a Fusion uses as material
    pub material_trap: Option<u32>,
    /// `s.material_race`
    pub material_race: Option<u64>,
    /// `s.material_location`
    pub material_location: Option<u32>,
    /// `s.max_metalmorph_stats={level, race}`
    pub max_metalmorph_stats: Option<(u32, u64)>,
    /// `s.rum_limit(mc, e)`: whether a Rank-Up-Magic may use `mc` (the monster to rank up) for this
    /// card (read as `c.rum_limit` by the Rank-Up-Magic scripts; absent = no limit)
    pub rum_limit: Option<fn(&mut ygo_core::duel::Duel, ygo_core::types::CardId, ygo_core::types::EffectId) -> bool>,
    /// `s.arcanareg(c, coin)` of the "Arcana Force" monsters: registers the effect of a coin result
    /// (read as `c.arcanareg` by "Arcana Call")
    pub arcanareg: Option<fn(&mut ygo_core::duel::Duel, ygo_core::types::CardId, i32)>,
}

impl ScriptInfo {
    pub const DEFAULT: ScriptInfo = ScriptInfo {
        maximum_side: None,
        self_equip_trap: false,
        loads: &[],
        listed_names: &[],
        fit_monster: &[],
        listed_card_types: &[],
        listed_series: &[],
        listed_cards: &[],
        material: &[],
        material_setcode: &[],
        tribute_req: None,
        counter_list: &[],
        counter_place_list: &[],
        dark_calling: false,
        pendulum_level: None,
        mat_filter: None,
        ritual_custom_check: None,
        roll_dice: false,
        toss_coin: false,
        lv: None,
        xyz_number: None,
        synchro_nt_required: None,
        synchro_tuner_required: None,
        miracle_synchro_fusion: false,
        assault_mode: None,
        self_tuner: false,
        material_trap: None,
        material_race: None,
        material_location: None,
        max_metalmorph_stats: None,
        rum_limit: None,
        arcanareg: None,
    };
}

impl Default for ScriptInfo {
    fn default() -> Self {
        ScriptInfo::DEFAULT
    }
}

/// The declarations of the script of `code`.
static DEFAULT_INFO: ScriptInfo = ScriptInfo::DEFAULT;
/// The codes of `cards::ALL` (sorted), compact for the binary search of `script_info`.
static CODES: std::sync::LazyLock<Vec<u32>> = std::sync::LazyLock::new(|| cards::ALL.iter().map(|x| x.0).collect());
pub fn script_info(code: u32) -> &'static ScriptInfo {
    match CODES.binary_search(&code) {
        Ok(i) => cards::ALL[i].2,
        Err(_) => &DEFAULT_INFO,
    }
}

/// The declarations of the script a card uses (its class table).
pub fn card_info(d: &ygo_core::duel::Duel, c: ygo_core::types::CardId) -> &'static ScriptInfo {
    script_info(d.card(c).script_code)
}

/// `c:GetMetatable(true)` (utility.lua): `_G["c"..c:GetCode()]`, the script table of the card's
/// *current* code (e.g. a name changed in the GY), which exists once a card with that code is in
/// the duel (its script was loaded); `None` otherwise.
pub fn current_info(d: &mut ygo_core::duel::Duel, c: ygo_core::types::CardId) -> Option<&'static ScriptInfo> {
    let code = *ygo_core::api::card::get_code(d, c).first()?;
    let loaded = (0..d.cards.len() as u32).map(ygo_core::types::CardId).any(|x| {
        let card = d.card(x);
        card.script_code == code || card.data.code == code || script_info(card.script_code).loads.contains(&code)
    });
    loaded.then(|| script_info(code))
}

/// A Lua runtime error in a script (`error(...)` or a type error).
pub fn lua_error(msg: &str) -> ! {
    ygo_core::lua::lua_error(msg)
}
