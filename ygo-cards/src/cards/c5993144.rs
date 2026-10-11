//! Swallow's Cowrie
use super::prelude::*;

pub const ID: u32 = 5993144;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 Winged Beast monster
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,tp)`
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:HasLevel() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetLevel())
    if !(card::is_race(d, c, RACE_WINGEDBEAST)
        && card::has_level(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND)))
    {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,lv)`
fn thfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:IsLevel(lv) and c:IsAbleToHand()
    card::is_race(d, c, RACE_WINGEDBEAST) && card::is_level(d, c, &[lv]) && card::is_able_to_hand(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,true,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, a.tp)), 1, 1, true, None, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local tc=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,true,nil,nil,tp):GetFirst()
    let sel = duel::select_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, a.tp)), 1, 1, true, None, NO_EX);
    let tc = group::get_first(d, &sel).nil();
    // e:SetLabel(tc:GetLevel())
    let lv = card::get_level(d, tc);
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.Release(tc,REASON_COST)
    duel::release(d, tc, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetLabel()==100 end
    if a.chk == Some(0) {
        return effect::get_label(d, a.e) == 100;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let lv = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
