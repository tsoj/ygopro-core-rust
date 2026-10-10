//! Cyber End Dragon
use super::prelude::*;

pub const ID: u32 = 1546123;

// s.material_setcode={SET_CYBER,SET_CYBER_DRAGON}
pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_CYBER, SET_CYBER_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,false,false,CARD_CYBER_DRAGON,3)
    fusion::add_proc_mix_n(d, c, false, false, vec![(fusion::Mat::Code(CARD_CYBER_DRAGON), 3)]);
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
