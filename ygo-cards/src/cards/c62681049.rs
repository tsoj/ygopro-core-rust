//! Dizzying Winds of Yosen Village
use super::prelude::*;

pub const ID: u32 = 62681049;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YOSENJU],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --todeck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EFFECT_TO_HAND_REDIRECT)
    effect::set_code(d, e2, EFFECT_TO_HAND_REDIRECT);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(effect_card_fn(tdtg)));
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetValue(LOCATION_DECKSHF)
    effect::set_value(d, e2, LOCATION_DECKSHF as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(6) and c:IsSetCard(SET_YOSENJU)
    card::is_faceup(d, c) && card::is_level_above(d, c, 6) && card::is_set_card(d, c, &[SET_YOSENJU as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tdtg(e,c)`
fn tdtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return (c:IsFacedown() or not c:IsSetCard(SET_YOSENJU)) and c:IsReason(REASON_EFFECT)
    (card::is_facedown(d, c) || !card::is_set_card(d, c, &[SET_YOSENJU as u16])) && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.tdcon(e)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsSetCard,e:GetHandlerPlayer(),LOCATION_PZONE,0,1,nil,SET_YOSENJU)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_YOSENJU as u16])), p, LOCATION_PZONE, 0, 1, NO_EX)
}
