//! Gust
use super::prelude::*;

pub const ID: u32 = 73079365;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn confilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpell() and c:IsPreviousLocation(LOCATION_SZONE) and c:IsPreviousControler(tp)
    //   and (c:GetReason()&(REASON_DESTROY|REASON_EFFECT))==(REASON_DESTROY|REASON_EFFECT)
    card::is_spell(d, c)
        && card::is_previous_location(d, c, LOCATION_SZONE)
        && card::is_previous_controler(d, c, tp)
        && (card::get_reason(d, c) & (REASON_DESTROY | REASON_EFFECT)) == (REASON_DESTROY | REASON_EFFECT)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and eg:IsExists(s.filter,1,nil,tp)
    let tp = a.tp;
    if a.rp != 1 - tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| confilter(d, c, tp), 1, NO_EX)
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and s.desfilter(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc) && Some(chkc) != effect::get_handler(d, a.e);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&desfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_target(d, tp, Some(&desfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
