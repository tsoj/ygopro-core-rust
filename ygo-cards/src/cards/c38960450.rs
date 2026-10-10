//! Armory Call
use super::prelude::*;

pub const ID: u32 = 38960450;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Equip Spell from your Deck to your hand, then you can equip it to 1 appropriate monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`: `return c:IsEquipSpell() and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_equip_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.eqfilter(c,ec)`: `return ec:CheckEquipTarget(c) and c:IsFaceup()`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    card::check_equip_target(d, ec, c) && card::is_faceup(d, c)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local ec=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let ec = group::get_first(d, &g);
    // if ec and Duel.SendtoHand(ec,nil,REASON_EFFECT)>0 and ec:IsLocation(LOCATION_HAND) then
    if let Some(ec) = ec {
        if duel::send_to_hand(d, ec, None, REASON_EFFECT, None) > 0 && card::is_location(d, ec, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,ec)
            duel::confirm_cards(d, 1 - a.tp, ec);
            // if not (ec:CheckUniqueOnField(tp) and not ec:IsForbidden()
            //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then return end
            if !(card::check_unique_on_field(d, ec, a.tp, None, None)
                && !card::is_forbidden(d, ec)
                && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
            {
                return;
            }
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // local g=Duel.GetMatchingGroup(s.eqfilter,tp,LOCATION_MZONE,0,nil,ec)
            let g = duel::get_matching_group(d, Some(&|d, x| eqfilter(d, x, ec)), a.tp, LOCATION_MZONE, 0, NO_EX);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
            if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
                // local sc=g:Select(tp,1,1,nil):GetFirst()
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                let sc = group::get_first(d, &sg);
                // if sc then
                if let Some(sc) = sc {
                    // Duel.HintSelection(sc)
                    duel::hint_selection(d, sc, true);
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.Equip(tp,ec,sc)
                    duel::equip(d, a.tp, ec, sc, true, false);
                }
            }
        }
    }
}
