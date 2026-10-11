//! Valkyrie's Embrace
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 45584727;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change 1 of your "Valkyrie" monsters to defense position, banish 1 of opponent's monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsSetCard(SET_VALKYRIE)
    card::is_set_card(d, a.c(0), &[SET_VALKYRIE as u16])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_VALKYRIE)
    !card::is_set_card(d, c, &[SET_VALKYRIE as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.NOT(Card.IsSetCard),tp,LOCATION_MZONE,0,1,nil,SET_VALKYRIE)
    !duel::is_existing_matching_card(d, Some(&|d, c| !card::is_set_card(d, c, &[SET_VALKYRIE as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VALKYRIE) and c:IsFaceup() and c:IsPosition(POS_FACEUP_ATTACK) and c:IsCanChangePosition()
    card::is_set_card(d, c, &[SET_VALKYRIE as u16])
        && card::is_faceup(d, c)
        && card::is_position(d, c, POS_FACEUP_ATTACK as u32)
        && card::is_can_change_position(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToRemove,nil),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g1=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g1), 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToRemove,nil),tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)),
        tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g2), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local ex1,tg1=Duel.GetOperationInfo(0,CATEGORY_POSITION)
    let tg1 = duel::get_operation_info(d, 0, CATEGORY_POSITION).and_then(|o| o.op_cards).nil();
    // local ex2,tg2=Duel.GetOperationInfo(0,CATEGORY_REMOVE)
    let tg2 = duel::get_operation_info(d, 0, CATEGORY_REMOVE).and_then(|o| o.op_cards).nil();
    // if tg1:GetFirst() and tg1:GetFirst():IsRelateToEffect(e) and Duel.ChangePosition(tg1,POS_FACEUP_DEFENSE)~=0
    //     and tg2:GetFirst() and tg2:GetFirst():IsRelateToEffect(e) then
    let Some(f1) = group::get_first(d, &tg1) else {
        return;
    };
    if !card::is_relate_to_effect(d, f1, e) {
        return;
    }
    if duel::change_position(d, tg1, POS_FACEUP_DEFENSE as u8, None, None, None, false) == 0 {
        return;
    }
    let Some(f2) = group::get_first(d, &tg2) else {
        return;
    };
    if card::is_relate_to_effect(d, f2, e) {
        // Duel.Remove(tg2,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tg2, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
