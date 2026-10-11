//! Genex Recycled
use super::prelude::*;

pub const ID: u32 = 51827737;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENEX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // name change
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and chkc:IsSetCard(SET_GENEX) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && card::is_set_card(d, chkc, &[SET_GENEX as u16]);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSetCard,tp,LOCATION_GRAVE,0,1,nil,SET_GENEX) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_set_card(d, c, &[SET_GENEX as u16])), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // Duel.SelectTarget(tp,Card.IsSetCard,tp,LOCATION_GRAVE,0,1,1,nil,SET_GENEX)
    duel::select_target(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_GENEX as u16])), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if not c:IsRelateToEffect(e) or c:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(tc:GetCode())
    let code = card::get_code(d, tc)[0];
    effect::set_value(d, e1, code as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
