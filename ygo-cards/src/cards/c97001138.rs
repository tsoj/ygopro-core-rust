//! Dark Advance
use super::prelude::*;

pub const ID: u32 = 97001138;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function() return Duel.IsMainPhase() or Duel.IsBattlePhase() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function() return Duel.IsMainPhase() or Duel.IsBattlePhase() end`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) || duel::is_battle_phase(d, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackAbove(2400) and c:GetDefense()==1000 and c:IsMonster() and c:IsAbleToHand()
    card::is_attack_above(d, c, 2400) && card::get_defense(d, c) == 1000 && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.sumfilter(c)`
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackAbove(2400) and c:GetDefense()==1000 and c:IsSummonable(true,nil,1)
    card::is_attack_above(d, c, 2400) && card::get_defense(d, c) == 1000 && card::is_summonable(d, c, true, None, 1, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0
        && card::is_location(d, tc, LOCATION_HAND)
    {
        // local g=Duel.GetMatchingGroup(s.sumfilter,tp,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&sumfilter), tp, LOCATION_HAND, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            // local sc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.Summon(tp,sc,true,nil,1)
            duel::summon(d, tp, sc, true, None, 1, None);
        } else {
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
        }
    }
}
