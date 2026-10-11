//! Spellbook of Fate
use super::prelude::*;

pub const ID: u32 = 97997309;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

// s.check is a script variable (s.check=false at load): key 0 of the card's s_var
const CHECK: u64 = 0;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.activate1)
    effect::set_operation(d, e1, Some(operation(activate1)));
    // e1:SetLabel(1)
    effect::set_label(d, e1, &[1]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_BATTLE_PHASE));
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.activate2)
    effect::set_operation(d, e2, Some(operation(activate2)));
    // e2:SetLabel(2)
    effect::set_label(d, e2, &[2]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e3, EFFECT_TYPE_ACTIVATE);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e3, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost_fn)));
    // e3:SetTarget(s.target3)
    effect::set_target(d, e3, Some(cost_target(target3)));
    // e3:SetOperation(s.activate3)
    effect::set_operation(d, e3, Some(operation(activate3)));
    // e3:SetLabel(3)
    effect::set_label(d, e3, &[3]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:IsSpell() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, _a: &EffArgs) -> bool {
    // s.check=true
    set_s_var(d, ID, CHECK, Value::Bool(true));
    // if chk==0 then return true end
    true
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToHand()
    card::is_facedown(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u32;
    if a.chk == Some(0) {
        // if not s.check then return false end
        if !s_var(d, ID, CHECK).truthy() {
            return false;
        }
        // s.check=false
        set_s_var(d, ID, CHECK, Value::Bool(false));
        // return Duel.IsExistingMatchingCard(s.rfilter,tp,LOCATION_GRAVE,0,ct,nil)
        //   and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_SZONE,LOCATION_SZONE,1,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&rfilter), a.tp, LOCATION_GRAVE, 0, ct, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter1), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, h.into());
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rfilter,tp,LOCATION_GRAVE,0,ct,ct,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&rfilter), a.tp, LOCATION_GRAVE, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_SZONE,LOCATION_SZONE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&filter1), a.tp, LOCATION_SZONE, LOCATION_SZONE, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// `s.activate1(e,tp,eg,ep,ev,re,r,rp)`
fn activate1(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let h = effect::get_handler(d, a.e).nil();
    // local g=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,e:GetHandler())
    let g = duel::select_matching_card(d, a.tp, Some(&filter1), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, h.into()).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.filter2(c)`: `return not c:IsPosition(POS_FACEUP_ATTACK) or c:IsCanTurnSet()`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    !card::is_position(d, c, POS_FACEUP_ATTACK) || card::is_can_turn_set(d, c)
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u32;
    if a.chk == Some(0) {
        // if not s.check then return false end
        if !s_var(d, ID, CHECK).truthy() {
            return false;
        }
        // s.check=false
        set_s_var(d, ID, CHECK, Value::Bool(false));
        // return Duel.IsExistingMatchingCard(s.rfilter,tp,LOCATION_GRAVE,0,ct,nil)
        //   and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        return duel::is_existing_matching_card(d, Some(&rfilter), a.tp, LOCATION_GRAVE, 0, ct, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter2), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rfilter,tp,LOCATION_GRAVE,0,ct,ct,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&rfilter), a.tp, LOCATION_GRAVE, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // local g=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter2), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, a.tp, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as i32);
    true
}
/// `s.activate2(e,tp,eg,ep,ev,re,r,rp)`
fn activate2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter2), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // if tc:IsPosition(POS_FACEUP_ATTACK) then
        if card::is_position(d, tc, POS_FACEUP_ATTACK) {
            // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        } else {
            // local pos=Duel.SelectPosition(tp,tc,POS_FACEUP_ATTACK+POS_FACEDOWN_DEFENSE)
            let pos = duel::select_position(d, a.tp, tc, (POS_FACEUP_ATTACK + POS_FACEDOWN_DEFENSE) as u8);
            // Duel.ChangePosition(tc,pos)
            duel::change_position(d, tc, pos as u8, None, None, None, false);
        }
    }
}
/// `s.target3(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target3(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u32;
    if a.chk == Some(0) {
        // if not s.check then return false end
        if !s_var(d, ID, CHECK).truthy() {
            return false;
        }
        // s.check=false
        set_s_var(d, ID, CHECK, Value::Bool(false));
        // return Duel.IsExistingMatchingCard(s.rfilter,tp,LOCATION_GRAVE,0,ct,nil)
        //   and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil)
        return duel::is_existing_matching_card(d, Some(&rfilter), a.tp, LOCATION_GRAVE, 0, ct, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rfilter,tp,LOCATION_GRAVE,0,ct,ct,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&rfilter), a.tp, LOCATION_GRAVE, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
/// `s.activate3(e,tp,eg,ep,ev,re,r,rp)`
fn activate3(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
