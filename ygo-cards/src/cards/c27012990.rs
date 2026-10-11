//! Marincess Cascade
use super::prelude::*;

pub const ID: u32 = 27012990;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase the ATK 1 face-up monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(lua_fn(actcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster() and c:IsSetCard(SET_MARINCESS) and c:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_faceup(d, c)
        && card::is_link_monster(d, c)
        && card::is_set_card(d, c, &[SET_MARINCESS as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::is_existing_target(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,sg)
    (duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, sg.into()), false)
}
// function s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local ct=99
    let mut ct: usize = 99;
    // local tgct=Duel.GetTargetCount(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let tgct = duel::get_target_count(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local mg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil,tp)
    let mg = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // if tgct==#mg then ct=#mg-1 end
    if tgct == mg.len() {
        ct = mg.len() - 1;
    }
    // if ct==0 then ct=1 end
    if ct == 0 {
        ct = 1;
    }
    // local g=aux.SelectUnselectGroup(mg,e,tp,1,ct,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil)
    let g = aux::select_unselect_group(d, &mg, Some(a.e), tp, Some(1), Some(ct), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // if Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_TEMPORARY)==#g then
    if duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST | REASON_TEMPORARY, None, None) == g.len() as i32 {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, a.e, Value::Group(g.clone()));
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // if Duel.IsPhase(PHASE_STANDBY) then e1:SetLabel(Duel.GetTurnCount()) end
        if duel::is_phase(d, PHASE_STANDBY) {
            let turn = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[turn]);
        }
        // e1:SetLabelObject(g)
        effect::set_label_object(d, e1, Value::Group(g.clone()));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY) then
        if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_STANDBY) {
            // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
    true
}
// function s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel()
    duel::is_turn_player(d, a.tp) && duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // for c in g:Iter() do Duel.ReturnToField(c) end
    for c in group::iter(d, &g) {
        duel::return_to_field(d, c, None, None);
    }
    // g:DeleteGroup()
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,0,e:GetLabelObject():GetSum(Card.GetLink)*300)
    let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let v = group::get_sum(d, &lg, &|d, c| card::get_link(d, c) as i64) * 300;
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g, 1, 0, v as i32);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabelObject():GetSum(Card.GetLink)*300)
        let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
        let v = group::get_sum(d, &lg, &|d, c| card::get_link(d, c) as i64) * 300;
        effect::set_value(d, e1, v);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// function s.actfilter(c)
fn actfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MARINCESS) and c:IsLinkMonster() and c:IsLinkAbove(3)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_link_monster(d, c) && card::is_link_above(d, c, 3)
}
// function s.actcon(e)
fn actcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.actfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let tp = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&actfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
