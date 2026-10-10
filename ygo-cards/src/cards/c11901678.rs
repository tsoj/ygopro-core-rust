//! Black Skull Dragon
use super::prelude::*;

pub const ID: u32 = 11901678;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_REDEYES_B_DRAGON],
    material_setcode: &[SET_RED_EYES, SET_ARCHFIEND],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_SUMMONED_SKULL,CARD_REDEYES_B_DRAGON)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_SUMMONED_SKULL), fusion::Mat::Code(CARD_REDEYES_B_DRAGON)]);
}
