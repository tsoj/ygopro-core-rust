//! Laval Forest Sprite
//! ラヴァル炎樹海の妖女
use super::prelude::*;

pub const ID: u32 = 15169262;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LAVAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to grave
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_LAVAL),tp,LOCATION_MZONE,0,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_LAVAL as u16]));
    let g = duel::get_matching_group(d, Some(&f), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local tc=g:GetFirst()
    // if not tc then return end
    if group::get_first(d, &g).is_none() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=Duel.GetMatchingGroupCount(Card.IsSetCard,tp,LOCATION_GRAVE,0,nil,SET_LAVAL)*200
    let atk = duel::get_matching_group_count(d, Some(&|d, c| card::is_set_card(d, c, &[SET_LAVAL as u16])), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 200;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
