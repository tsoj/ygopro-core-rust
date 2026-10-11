//! Spell Power Mastery
use super::prelude::*;

pub const ID: u32 = 38943357;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[75014062],
    listed_series: &[SET_ENDYMION],
    counter_place_list: &[COUNTER_SPELL as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Endymion" card from your Deck to your hand, then you can count the number of ...
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ENDYMION) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ENDYMION as u16]) && card::is_able_to_hand(d, c, None)
}
/// `aux.FaceupFilter(Card.IsCode,id,75014062)`
fn faceup_code(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[ID, 75014062])
}
/// `aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1)`
fn faceup_can_add_spell(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsCode,id,75014062),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
        let mut ct = duel::get_matching_group_count(d, Some(&faceup_code), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
        // local cg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1),tp,LOCATION_ONFIELD,0,nil)
        let mut cg = duel::get_matching_group(d, Some(&faceup_can_add_spell), tp, LOCATION_ONFIELD, 0, NO_EX);
        // if ct>0 and #cg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if ct > 0 && !cg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // while ct>0 and #cg>0 do
            while ct > 0 && !cg.is_empty() {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_COUNTER as u64);
                // cg:Select(tp,1,1,nil):GetFirst():AddCounter(COUNTER_SPELL,1)
                let sel = group::select(d, &cg, tp, 1, 1, false, NO_EX).nil();
                let tc = group::get_first(d, &sel).nil();
                card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
                // ct=ct-1
                ct -= 1;
                // cg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1),tp,LOCATION_ONFIELD,0,nil)
                cg = duel::get_matching_group(d, Some(&faceup_can_add_spell), tp, LOCATION_ONFIELD, 0, NO_EX);
            }
        }
    }
}
