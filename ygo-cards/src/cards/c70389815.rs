//! Amazing Time Ticket
use super::prelude::*;

pub const ID: u32 = 70389815;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ATTRACTION, SET_AMAZEMENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The same arguments with `chk` replaced (`s.thtg(e,tp,...,0)` and friends).
fn with_chk(a: &EffArgs, chk: i64) -> EffArgs {
    let mut b = a.clone();
    b.chk = Some(chk);
    b
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.IsTurnPlayer(tp) then return s.thtg(e,tp,eg,ep,ev,re,r,rp,0)
        if duel::is_turn_player(d, a.tp) {
            return thtg(d, &with_chk(a, 0));
        } else {
            // else return s.settg(e,tp,eg,ep,ev,re,r,rp,0) end
            return settg(d, &with_chk(a, 0));
        }
    }
    // if Duel.IsTurnPlayer(tp) then
    if duel::is_turn_player(d, a.tp) {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e:SetOperation(s.thop)
        effect::set_operation(d, a.e, Some(operation(thop)));
        // s.thtg(e,tp,eg,ep,ev,re,r,rp,1)
        thtg(d, &with_chk(a, 1));
    } else {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SET);
        // e:SetOperation(s.setop)
        effect::set_operation(d, a.e, Some(operation(setop)));
        // s.settg(e,tp,eg,ep,ev,re,r,rp,1)
        settg(d, &with_chk(a, 1));
    }
    true
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_AMAZEMENT) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_AMAZEMENT as u16]) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
// s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ATTRACTION) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_ATTRACTION as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
// s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SSet(tp,tc)
        duel::s_set(d, tp, tc, None, true);
        // local e0=Effect.CreateEffect(tc)
        let e0 = effect::create_effect(d, tc);
        // e0:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e0, aux::stringid(ID, 0));
        // e0:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        // e0:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
        effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_SET_TURN);
        // e0:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e0, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e0)
        card::register_effect(d, tc, e0, false);
    }
}
