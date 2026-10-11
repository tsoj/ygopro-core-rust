//! X・HERO ドレッドバスター
//! Xtra HERO Dread Decimator
use super::prelude::*;

pub const ID: u32 = 63813056;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_HERO),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_HERO as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
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
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return (e:GetHandler():GetLinkedGroup():IsContains(c) or c==e:GetHandler()) and c:IsSetCard(SET_HERO) and c:IsFaceup()
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    (lg.contains(c) || c == h) && card::is_set_card(d, c, &[SET_HERO as u16]) && card::is_faceup(d, c)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_HERO) and c:IsMonster()
    card::is_set_card(d, c, &[SET_HERO as u16]) && card::is_type(d, c, TYPE_MONSTER)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroup(s.atkfilter,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil):GetClassCount(Card.GetCode)*100
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&atkfilter), p, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64 * 100
}
