//! Subterror Final Battle
use super::prelude::*;

pub const ID: u32 = 74640994;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUBTERROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_END_PHASE));
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SUBTERROR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUBTERROR as u16])
}
/// `s.fufilter(c)`
fn fufilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_SUBTERROR)
    card::is_facedown(d, c) && card::is_set_card(d, c, &[SET_SUBTERROR as u16])
}
/// `s.fdfilter(c)`
fn fdfilter(d: &mut Duel, c: CardId) -> bool {
    // return s.filter(c) and c:IsCanTurnSet()
    filter(d, c) && card::is_can_turn_set(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.GetCurrentPhase()~=PHASE_DAMAGE and Duel.IsExistingMatchingCard(s.fufilter,tp,LOCATION_MZONE,0,1,nil)
    let b1 = duel::get_current_phase(d) != PHASE_DAMAGE && duel::is_existing_matching_card(d, Some(&fufilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.GetCurrentPhase()~=PHASE_DAMAGE and Duel.IsExistingMatchingCard(s.fdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b2 = duel::get_current_phase(d) != PHASE_DAMAGE && duel::is_existing_matching_card(d, Some(&fdfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b3 = duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // local b4=Duel.GetCurrentPhase()~=PHASE_DAMAGE and Duel.GetFlagEffect(tp,id)==0
    let b4 = duel::get_current_phase(d) != PHASE_DAMAGE && duel::get_flag_effect(d, tp, ID) == 0;
    // if chk==0 then return b1 or b2 or b3 or b4 end
    if a.chk == Some(0) {
        return b1 || b2 || b3 || b4;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)}, {b3,aux.Stringid(id,2)}, {b4,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1)), (b3, aux::stringid(ID, 2)), (b4, aux::stringid(ID, 3))],
    );
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    // if op==1 or op==2 then
    if op == Some(1) || op == Some(2) {
        // e:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_POSITION + CATEGORY_SET);
        // local pos=op==1 and (POS_FACEUP_ATTACK|POS_FACEUP_DEFENSE) or POS_FACEDOWN_DEFENSE
        let pos = if op == Some(1) { POS_FACEUP_ATTACK | POS_FACEUP_DEFENSE } else { POS_FACEDOWN_DEFENSE };
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,tp,pos)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, tp, pos as i32);
    // elseif op==3 then
    } else if op == Some(3) {
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_SET);
    }
    true
}
/// `s.effectfilter(e,ct)`
fn effectfilter(d: &mut Duel, a: Args) -> bool {
    // local te=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT)
    let te = duel::get_chain_info(d, a.int(1) as u8, &[CHAININFO_TRIGGERING_EFFECT]).first().and_then(|v| v.as_effect()).nil();
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te).nil();
    // return tc:IsSetCard(SET_SUBTERROR)
    card::is_set_card(d, tc, &[SET_SUBTERROR as u16])
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.fufilter,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&fufilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local tc=g:GetFirst()
            let tc = group::get_first(d, &g).nil();
            // local pos=Duel.SelectPosition(tp,tc,POS_FACEUP_ATTACK+POS_FACEUP_DEFENSE)
            let pos = duel::select_position(d, tp, tc, (POS_FACEUP_ATTACK | POS_FACEUP_DEFENSE) as u8);
            // Duel.ChangePosition(tc,pos)
            duel::change_position(d, CardSet::from_card(tc), pos as u8, None, None, None, false);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.fdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&fdfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    // elseif op==3 then
    } else if op == 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local tc=g:GetFirst()
            let tc = group::get_first(d, &g).nil();
            // local atk=tc:GetBaseAttack()+tc:GetBaseDefense()
            let atk = card::get_base_attack(d, tc) + card::get_base_defense(d, tc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(atk)
            effect::set_value(d, e1, atk as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    } else {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_DISEFFECT)
        effect::set_code(d, e1, EFFECT_CANNOT_DISEFFECT);
        // e1:SetValue(s.effectfilter)
        effect::set_value_fn(d, e1, lua_fn(effectfilter));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if c:IsSSetable(true) then
    if card::is_s_setable(d, c, true, None) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.ChangePosition(c,POS_FACEDOWN)
        duel::change_position(d, CardSet::from_card(c), POS_FACEDOWN as u8, None, None, None, false);
        // Duel.RaiseEvent(c,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, CardSet::from_card(c), EVENT_SSET, Some(a.e), REASON_EFFECT, tp, tp, 0);
    }
}
