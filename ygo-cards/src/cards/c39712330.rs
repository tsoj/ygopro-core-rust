//! Cry Havoc!
use super::prelude::*;

pub const ID: u32 = 39712330;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // instant
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.activate)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsMainPhase()
    duel::is_turn_player(d, a.tp) && duel::is_main_phase(d, None)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:CanSummonOrSet(true,nil)
    card::is_type(d, c, TYPE_NORMAL) && card::can_summon_or_set(d, c, true, None, 0, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // if e:GetHandler():GetFlagEffect(id)==0 then
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) == 0 {
        // e:GetHandler():RegisterFlagEffect(id,RESET_CHAIN,0,1,Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil))
        let n = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
        card::register_flag_effect(d, h, ID, RESET_CHAIN, 0, 1, n as i64, 0);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetHandler():GetFlagEffect(id)==0 then
        if card::get_flag_effect(d, h, ID) == 0 {
            // return Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)>0
            return duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX) > 0;
        }
        // else return e:GetHandler():GetFlagEffectLabel(id)>0 end
        return card::get_flag_effect_label(d, h, ID).first().copied().nil() > 0;
    }
    // e:GetHandler():SetFlagEffectLabel(id,e:GetHandler():GetFlagEffectLabel(id)-1)
    let lab = card::get_flag_effect_label(d, h, ID).first().copied().nil();
    card::set_flag_effect_label(d, h, ID, (lab - 1) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SummonOrSet(tp,tc,true,nil)
        duel::summon_or_set(d, a.tp, tc, true, None, 0, None);
    }
}
