//! The card scripts (`external/CardScripts`) in Rust: the global scripts (`utility.lua`, the
//! `proc_*.lua` procedures, the rules they add), and one module per card.
//!
//! Scripts call the API through `lua` (the core's API with the changes the global Lua scripts
//! make to it), write card functions with `prelude`, and use the constants of `constant.lua`.

pub mod constants;
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

/// What a card's script declares besides its effects (fields of the Lua card table).
#[derive(Clone, Copy, Default, Debug)]
pub struct ScriptInfo {
    pub maximum_side: Option<global::MaximumSide>,
}

/// The declarations of the script of `code`.
pub fn script_info(_code: u32) -> ScriptInfo {
    ScriptInfo::default()
}
