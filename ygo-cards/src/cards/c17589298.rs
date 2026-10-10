//! Sword of Dark Rites
use super::prelude::*;

pub const ID: u32 = 17589298;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_attribute(d, c, ATTRIBUTE_DARK))), None, None, None, None, None, None);
    // Atk update
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 400);
    card::register_effect(d, c, e1, false);
    // tohand
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(retcon)));
    effect::set_target(d, e2, Some(cost_target(rettg)));
    effect::set_operation(d, e2, Some(operation(retop)));
    card::register_effect(d, c, e2, false);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec:IsReason(REASON_RELEASE)
    card::is_reason(d, c, REASON_LOST_TARGET) && card::is_reason(d, ec.nil(), REASON_RELEASE)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_hand(d, h, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, c);
    }
}
