//! F.G.D.
use super::prelude::*;

pub const ID: u32 = 99267150;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),5)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))), 5)],
    );
    // battle indestructable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(s.batfilter)
    effect::set_value_fn(d, e2, effect_card_fn(batfilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon condition
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // e3:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e3, aux::fuslimit());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.batfilter(e,c)`
fn batfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_WIND|ATTRIBUTE_FIRE|ATTRIBUTE_WATER|ATTRIBUTE_EARTH)
    card::is_attribute(d, c, ATTRIBUTE_DARK | ATTRIBUTE_WIND | ATTRIBUTE_FIRE | ATTRIBUTE_WATER | ATTRIBUTE_EARTH)
}
