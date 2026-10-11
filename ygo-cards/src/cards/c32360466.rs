//! Beginning of Heaven and Earth
use super::prelude::*;

pub const ID: u32 = 32360466;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACK_LUSTER_SOLDIER, SET_GAIA_THE_FIERCE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
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
    // return c:IsRace(RACE_WARRIOR) and c:IsAbleToHand()
    card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_hand(d, c, None)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_BLACK_LUSTER_SOLDIER,SET_GAIA_THE_FIERCE_KNIGHT})
    card::is_set_card(d, c, &[SET_BLACK_LUSTER_SOLDIER as u16, SET_GAIA_THE_FIERCE_KNIGHT as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| filter1(d, c)), tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return #g>2 and g:IsExists(s.filter2,1,nil) end
    if a.chk == Some(0) {
        return g.len() > 2 && group::is_exists(d, &g, &|d, c| filter2(d, c), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(s.filter2,1,nil)
    (group::is_exists(d, sg, &|d, c| filter2(d, c), 1, NO_EX), false)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| filter1(d, c)), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_ATOHAND)
    let mut sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
        // local tc=sg:Select(1-tp,1,1,nil):GetFirst()
        let sel = group::select(d, &sg, 1 - tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.Hint(HINT_CARD,0,tc:GetCode())
        let code = card::get_code(d, tc)[0];
        duel::hint(d, HINT_CARD as u8, 0, code as u64);
        // if s.filter2(tc) and tc:IsAbleToHand() then
        if filter2(d, tc) && card::is_able_to_hand(d, tc, None) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
            // sg:RemoveCard(tc)
            group::remove_card(&mut sg, &CardSet::from_card(tc));
        }
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
