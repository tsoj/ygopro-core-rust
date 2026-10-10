//! The card scripts: one module per script, `c<code>.rs` (`official/c<code>.lua`). Each has
//! `pub const ID: u32` (the code), `pub fn initial_effect(d, c)`, and if the Lua card table has
//! declarations (`s.listed_names`, ...), `pub const INFO: ScriptInfo`. `build.rs` lists them.

use crate::ScriptInfo;
use ygo_core::duel::Duel;
use ygo_core::types::CardId;

/// What a card module imports (`use super::prelude::*;`).
pub mod prelude {
    pub use crate::constants::*;
    pub use crate::lua::{card, debug, duel, effect, group, CardOrGroup, Exclude, Filter, NO_EX};
    pub use crate::prelude::*;
    pub use crate::procs::{fusion, gemini, group_check, link, mat_filter, pendulum, ritual, spirit, synchro, xyz, GroupCheck, MatFilter};
    pub use crate::{aux, chain, cost, ScriptInfo};
}

include!(concat!(env!("OUT_DIR"), "/cards.rs"));
