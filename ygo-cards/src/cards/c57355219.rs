//! Paradox Fusion
use super::prelude::*;

pub const ID: u32 = 57355219;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SPSUMMON);
    effect::set_condition(d, e1, Some(condition(condition1)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target1)));
    effect::set_operation(d, e1, Some(operation(activate1)));
    card::register_effect(d, c, e1, false);
    // Activate(effect)
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_condition(d, e2, Some(condition(condition2)));
    effect::set_cost(d, e2, Some(cost_target(cost)));
    effect::set_target(d, e2, Some(cost_target(target2)));
    effect::set_operation(d, e2, Some(operation(activate2)));
    card::register_effect(d, c, e2, false);
}
fn condition1(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0
    duel::get_current_chain(d, true) == 0
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:IsAbleToRemoveAsCost() and not c:IsStatus(STATUS_BATTLE_DESTROYED)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_able_to_remove_as_cost(d, c, None)
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST + REASON_TEMPORARY, None, None);
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::from));
    true
}
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), n, 0, 0);
    true
}
fn activate1(d: &mut Duel, a: &EffArgs) {
    duel::negate_summon(d, a.eg());
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    return_effect(d, a);
}
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8)
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate2(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    if duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
    return_effect(d, a);
}
/// The part of Activate(summon) and Activate(effect) that follows the negation: the card returns to
/// the field two turns later (`EVENT_PHASE+PHASE_END` with a turn counter on the card).
fn return_effect(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    effect::set_label_object(d, e1, tc.clone());
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(retcon)));
    effect::set_operation(d, e1, Some(operation(retop)));
    // tc:SetTurnCounter(0)
    card::set_turn_counter(d, tc.as_card().nil(), 0);
    duel::register_effect(d, e1, a.tp);
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
fn retop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetLabelObject()
    // local ct=c:GetTurnCounter()
    // c:SetTurnCounter(ct+1)
    // if ct==1 then Duel.ReturnToField(c,POS_FACEUP_ATTACK) end
    let c = effect::get_label_object(d, a.e).as_card().nil();
    let ct = card::get_turn_counter(d, c);
    card::set_turn_counter(d, c, (ct + 1) as u16);
    if ct == 1 {
        duel::return_to_field(d, c, Some(POS_FACEUP_ATTACK as u8), None);
    }
}
