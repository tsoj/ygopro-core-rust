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
    /// `s.listed_names`
    pub listed_names: &'static [u32],
    /// `s.fit_monster`
    pub fit_monster: &'static [u32],
    /// `s.listed_card_types`
    pub listed_card_types: &'static [u32],
    /// `s.listed_series`
    pub listed_series: &'static [u32],
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
}

impl ScriptInfo {
    pub const DEFAULT: ScriptInfo = ScriptInfo {
        maximum_side: None,
        self_equip_trap: false,
        listed_names: &[],
        fit_monster: &[],
        listed_card_types: &[],
        listed_series: &[],
        material: &[],
        material_setcode: &[],
        tribute_req: None,
        counter_list: &[],
        counter_place_list: &[],
        dark_calling: false,
    };
}

impl Default for ScriptInfo {
    fn default() -> Self {
        ScriptInfo::DEFAULT
    }
}

/// The declarations of the script of `code`.
pub fn script_info(code: u32) -> ScriptInfo {
    match cards::ALL.binary_search_by_key(&code, |x| x.0) {
        Ok(i) => *cards::ALL[i].2,
        Err(_) => ScriptInfo::DEFAULT,
    }
}

/// The declarations of the script a card uses (its class table).
pub fn card_info(d: &ygo_core::duel::Duel, c: ygo_core::types::CardId) -> ScriptInfo {
    script_info(d.card(c).script_code)
}

/// A Lua runtime error in a script (`error(...)` or a type error).
pub fn lua_error(msg: &str) -> ! {
    ygo_core::lua::lua_error(msg)
}
