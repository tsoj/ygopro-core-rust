//! Dimensionhole
use super::prelude::*;

pub const ID: u32 = 22959079;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove()
    card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
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
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local seq=tc:GetSequence()
    let mut seq = card::get_sequence(d, tc);
    // if tc:IsControler(1-tp) then seq=seq+16 end
    if card::is_controler(d, tc, 1 - tp) {
        seq += 16;
    }
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h1 = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h1);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, 2);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.rtcon)
        effect::set_condition(d, e1, Some(condition(rtcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let h2 = effect::get_handler(d, a.e).nil();
        let e2 = effect::create_effect(d, h2);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_DISABLE_FIELD)
        effect::set_code(d, e2, EFFECT_DISABLE_FIELD);
        // e2:SetLabel(seq)
        effect::set_label(d, e2, &[seq as i64]);
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // e2:SetCondition(s.discon)
        effect::set_condition(d, e2, Some(lua_fn(discon)));
        // e2:SetOperation(s.disop)
        effect::set_operation(d, e2, Some(lua_fn(disop)));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.rtcon(e,tp,eg,ep,ev,re,r,rp)`
fn rtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, tc, None, None);
}
/// `s.discon(e,c)`
fn discon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if e:GetLabelObject():IsLocation(LOCATION_REMOVED) then
    let tc = effect::get_label_object(d, e).as_card().nil();
    if card::is_location(d, tc, LOCATION_REMOVED) {
        // return true
        return true;
    }
    // e:Reset()
    effect::reset(d, e);
    // return false
    false
}
/// `s.disop(e,tp)`
fn disop(d: &mut Duel, a: Args) -> u32 {
    // local dis1=(0x1<<e:GetLabel())
    // return dis1
    let shift = effect::get_label(d, a.e(0)) as u32;
    1u32 << shift
}
