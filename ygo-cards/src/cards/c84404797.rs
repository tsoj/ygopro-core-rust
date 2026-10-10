//! Redeemable Jar
use super::prelude::*;

pub const ID: u32 = 84404797;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Add to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c,code)`
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_trap(d, c) && !card::is_original_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsTrap() and c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,c,c:GetOriginalCode())
    if !(card::is_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)) {
        return false;
    }
    let code = card::get_original_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, code)), tp, LOCATION_GRAVE, 0, 1, c.into())
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local tg=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tg = group::get_first(d, &g).nil();
    // Duel.Remove(tg,POS_FACEUP,REASON_COST)
    duel::remove(d, tg, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(tg:GetOriginalCode())
    let code = card::get_original_code(d, tg);
    effect::set_label(d, a.e, &[code as i64]);
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,1,nil,e:GetLabel())
    let label = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x, label))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
