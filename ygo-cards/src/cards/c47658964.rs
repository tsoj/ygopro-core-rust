//! Searing Fire Wall
use super::prelude::*;

pub const ID: u32 = 47658964;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[47658965], listed_series: &[SET_LAVAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LAVAL) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_LAVAL as u16]) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `{MonsterData}` of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_LAVAL,TYPES_TOKEN,0,0,1,RACE_PYRO,ATTRIBUTE_FIRE)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_LAVAL as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_PYRO),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    let g = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        // g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE,0,1,5,nil)
        duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, 5, false, NO_EX).nil()
    } else {
        // g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,Duel.GetLocationCount(tp,LOCATION_MZONE),nil)
        let max = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as u16;
        duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, max, false, NO_EX).nil()
    };
    // e:SetLabel(#g)
    effect::set_label(d, e, &[g.len() as i64]);
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 or Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION))
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_LAVAL,TYPES_TOKEN,0,0,1,RACE_PYRO,ATTRIBUTE_FIRE) end
    if a.chk == Some(0) {
        let slot = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            || duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some();
        return slot && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,e:GetLabel(),0,0)
    let n = effect::get_label(d, e) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,e:GetLabel(),tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, n, tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let label = effect::get_label(d, e);
    // if ft<e:GetLabel() or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if ft < label || !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // for i=1,e:GetLabel() do
    for _ in 0..label {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
