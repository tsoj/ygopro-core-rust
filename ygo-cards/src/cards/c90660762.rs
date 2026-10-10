//! Meteor Black Dragon
use super::prelude::*;

pub const ID: u32 = 90660762;

// s.listed_names={CARD_REDEYES_B_DRAGON}
// s.material_setcode=SET_RED_EYES
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_RED_EYES], listed_names: &[CARD_REDEYES_B_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_REDEYES_B_DRAGON,64271667)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_REDEYES_B_DRAGON), fusion::Mat::Code(64271667)]);
}
