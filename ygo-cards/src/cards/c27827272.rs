//! Makiu, the Magical Mist
use super::prelude::*;

pub const ID: u32 = 27827272;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SUMMONED_SKULL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsPhase(PHASE_MAIN1) end
    if a.chk == Some(0) {
        return duel::is_phase(d, PHASE_MAIN1);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_SUMMONED_SKULL) or c:IsRace(RACE_THUNDER))
    //   and Duel.IsExistingMatchingCard(s.filter2,tp,0,LOCATION_MZONE,1,nil,c:GetAttack())
    if !card::is_faceup(d, c) || !(card::is_code(d, c, &[CARD_SUMMONED_SKULL]) || card::is_race(d, c, RACE_THUNDER)) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.filter2(c,atk)`
fn filter2(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsDefenseBelow(atk)
    card::is_faceup(d, c) && card::is_defense_below(d, c, atk)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local dg=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_MZONE,nil,g:GetFirst():GetAttack())
    let tc = group::get_first(d, &g).nil();
    let atk = card::get_attack(d, tc);
    let dg = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, atk)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local dg=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_MZONE,nil,tc:GetAttack())
        let atk = card::get_attack(d, tc);
        let dg = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, atk)), a.tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
