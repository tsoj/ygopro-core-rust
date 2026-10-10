//! Yomi Ship
use super::prelude::*;

pub const ID: u32 = 51534754;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    //     and e:GetHandler():GetReasonCard():IsRelateToBattle()
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_GRAVE) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_reason(d, c, REASON_BATTLE) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, c).nil();
    card::is_relate_to_battle(d, rc)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=e:GetHandler():GetReasonCard()
    let c = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, c).nil();
    // Duel.SetTargetCard(rc)
    duel::set_target_card(d, rc);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,rc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(rc)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local rc=Duel.GetFirstTarget()
    let rc = duel::first_target(d).nil();
    // if rc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, rc, a.e) {
        // Duel.Destroy(rc,REASON_EFFECT)
        duel::destroy(d, rc, REASON_EFFECT, None, None);
    }
}
