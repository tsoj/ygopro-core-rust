//! Phantom of Chaos
use super::prelude::*;

pub const ID: u32 = 30312361;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish a monster and this card's name/original ATK become that monster's, and replace this effect with that monster's original effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent takes no battle damage from attacks involving this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_BATTLE_DAMAGE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not e:GetHandler():HasFlagEffect(id) end
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None);
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EFFECT) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_type(d, c, TYPE_EFFECT) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, e)
    {
        // local code=tc:GetOriginalCode()
        let code = card::get_original_code(d, tc);
        // local atk=tc:GetBaseAttack()
        let atk = card::get_base_attack(d, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetValue(code)
        effect::set_value(d, e1, code as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
        // e2:SetValue(atk)
        effect::set_value(d, e2, atk as i64);
        // e2:SetLabelObject(e1)
        effect::set_label_object(d, e2, Value::Effect(e1));
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // local cid=c:CopyEffect(code,RESETS_STANDARD_PHASE_END,1)
        let cid = card::copy_effect(d, c, code, RESETS_STANDARD_PHASE_END, 1);
        // Reset the effects manually (to handle interactions with mandatory effects)
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e3, aux::stringid(ID, 1));
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3, LOCATION_MZONE);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetLabel(cid)
        effect::set_label(d, e3, &[cid as i64]);
        // e3:SetLabelObject(e1)
        effect::set_label_object(d, e3, Value::Effect(e1));
        // e3:SetOperation(s.resettop)
        effect::set_operation(d, e3, Some(operation(resettop)));
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
/// `s.resettop(e,tp,eg,ep,ev,re,r,rp)`
fn resettop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local cid=e:GetLabel()
    let cid = effect::get_label(d, e);
    // c:ResetEffect(cid,RESET_COPY)
    card::reset_effect(d, c, cid as u32, RESET_COPY);
    // c:ResetEffect(RESET_DISABLE,RESET_EVENT)
    card::reset_effect(d, c, RESET_DISABLE, RESET_EVENT);
    // local e1=e:GetLabelObject()
    // e1:Reset()
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    effect::reset(d, e1);
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, CardSet::from_card(c), true);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
}
