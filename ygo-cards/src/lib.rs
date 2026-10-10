//! The card scripts (`external/CardScripts`) in Rust: the global scripts (`utility.lua`, the
//! `proc_*.lua` procedures, the rules they add), and one module per card.
//!
//! Scripts call the API through `lua` (the core's API with the changes the global Lua scripts
//! make to it), write card functions with `prelude`, and use the constants of `constant.lua`.

pub mod aux;
pub mod chain;
pub mod constants;
pub mod cost;
pub mod global;
pub mod lua;
pub mod prelude;

use ygo_core::script::Scripts;

/// All card scripts (code -> initial_effect).
pub fn scripts() -> Scripts {
    Scripts::default()
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
    };
}

impl Default for ScriptInfo {
    fn default() -> Self {
        ScriptInfo::DEFAULT
    }
}

/// The declarations of the script of `code`.
pub fn script_info(_code: u32) -> ScriptInfo {
    ScriptInfo::DEFAULT
}

/// The declarations of the script a card uses (its class table).
pub fn card_info(d: &ygo_core::duel::Duel, c: ygo_core::types::CardId) -> ScriptInfo {
    script_info(d.card(c).script_code)
}

/// A Lua runtime error in a script (`error(...)` or a type error).
pub fn lua_error(msg: &str) -> ! {
    ygo_core::lua::lua_error(msg)
}
