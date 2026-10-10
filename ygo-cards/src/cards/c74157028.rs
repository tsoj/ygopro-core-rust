//! Cyber Twin Dragon
use super::prelude::*;

pub const ID: u32 = 74157028;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_CYBER, SET_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,false,false,CARD_CYBER_DRAGON,2)
    fusion::add_proc_mix_n(d, c, false, false, vec![(fusion::Mat::Code(CARD_CYBER_DRAGON), 2)]);
    // extra att
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
