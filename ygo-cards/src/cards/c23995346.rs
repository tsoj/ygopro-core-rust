//! Blue-Eyes Ultimate Dragon
use super::prelude::*;

pub const ID: u32 = 23995346;
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_BLUE_EYES], listed_names: &[CARD_BLUEEYES_W_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Procedure
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::Code(CARD_BLUEEYES_W_DRAGON), 3)]);
}
