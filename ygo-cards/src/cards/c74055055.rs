//! Oil
use super::prelude::*;

pub const ID: u32 = 74055055;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add up to 2 level 4 or lower FIRE monsters to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
}
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // c:IsLevelBelow(4) and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_level_below(d, c, 4) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc, a.e);
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let dn = aux::dncheck();
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&dn), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), n, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
