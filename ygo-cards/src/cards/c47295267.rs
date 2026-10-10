//! Cybernetic Zone
use super::prelude::*;

pub const ID: u32 = 47295267;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 Machine Fusion Monster you control until the End Phase
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsType(TYPE_FUSION) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_type(d, c, TYPE_FUSION) && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, a.tp, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if Duel.ReturnToField(e:GetLabelObject()) then
    if duel::return_to_field(d, tc, None, None) {
        // local e1=Effect.CreateEffect(e:GetOwner())
        let owner = effect::get_owner(d, a.e).nil();
        let e1 = effect::create_effect(d, owner);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetAttack()*2)
        let atk = card::get_attack(d, tc) as i64;
        effect::set_value(d, e1, atk * 2);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetOwner())
        let e2 = effect::create_effect(d, owner);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetCondition(s.descon)
        effect::set_condition(d, e2, Some(condition(descon)));
        // e2:SetOperation(s.desop)
        effect::set_operation(d, e2, Some(operation(desop)));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():IsRace(RACE_MACHINE)
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && card::is_race(d, h, RACE_MACHINE)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
