//! Rikka Glamour
use super::prelude::*;

pub const ID: u32 = 69164989;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RIKKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
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
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8, check: i64) -> bool {
    // return c:IsSetCard(SET_RIKKA) and c:IsMonster() and c:IsAbleToHand()
    //     and (check==0 or Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil,c:GetOriginalLevel(),c:GetCode()))
    if !(card::is_set_card(d, c, &[SET_RIKKA as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)) {
        return false;
    }
    if check == 0 {
        return true;
    }
    let lv = card::get_original_level(d, c);
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, tc| filter2(d, tc, lv, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn filter2(d: &mut Duel, c: CardId, lv: u32, code: u32) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsMonster() and c:IsAbleToHand() and c:GetOriginalLevel()==lv and not c:IsCode(code)
    card::is_race(d, c, RACE_PLANT)
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && card::get_original_level(d, c) == lv
        && !card::is_code(d, c, &[code])
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,0)
    let ca = duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp, 0)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,1)
    let cb = duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp, 1)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return a or b end
    if a.chk == Some(0) {
        return ca || cb;
    }
    // if b and Duel.CheckReleaseGroupCost(tp,Card.IsRikkaReleasable,1,false,nil,nil,tp) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if cb
        && duel::check_release_group_cost(d, a.tp, Some(&|d, tc| aux::is_rikka_releasable(d, tc, a.tp)), 1, 1, false, None, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // local g=Duel.SelectReleaseGroupCost(tp,Card.IsRikkaReleasable,1,1,false,nil,nil,tp)
        let g = duel::select_release_group_cost(d, a.tp, Some(&|d, tc| aux::is_rikka_releasable(d, tc, a.tp)), 1, 1, false, None, NO_EX);
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,0) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp, 0)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,e:GetLabel()+1,tp,LOCATION_DECK)
    let lbl = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, (lbl + 1) as u8, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp,e:GetLabel())
    let lbl = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.tp, lbl)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local lv=g:GetFirst():GetOriginalLevel()
        let tc = group::get_first(d, &g).nil();
        let lv = card::get_original_level(d, tc);
        // local code=g:GetFirst():GetCode()
        let code = card::get_code(d, tc)[0];
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // if e:GetLabel()==1 then
        if effect::get_label(d, a.e) == 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,lv,code)
            let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| filter2(d, c, lv, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - a.tp, sg);
            }
        }
    }
}
