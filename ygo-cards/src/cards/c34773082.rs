//! Frightfur Patchwork
use super::prelude::*;

pub const ID: u32 = 34773082;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EDGE_IMP], listed_names: &[CARD_POLYMERIZATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_able_to_hand(d, c, None)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EDGE_IMP) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EDGE_IMP as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter1), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter2), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil)
    let g1 = duel::get_matching_group(d, Some(&filter1), a.tp, LOCATION_DECK, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_DECK,0,nil)
    let g2 = duel::get_matching_group(d, Some(&filter2), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg1=g1:Select(tp,1,1,nil)
        let mut sg1 = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg2=g2:Select(tp,1,1,nil)
        let sg2 = group::select(d, &g2, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // Duel.SendtoHand(sg1,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg1.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg1)
        duel::confirm_cards(d, 1 - a.tp, sg1);
    }
}
