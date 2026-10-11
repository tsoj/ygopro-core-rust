//! Prevention Star
use super::prelude::*;

pub const ID: u32 = 94303232;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetValue(s.eqlimit)
    effect::set_value_fn(d, e2, effect_card_fn(eqlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Remove
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // e3:SetLabelObject(e1)
    effect::set_label_object(d, e3, Value::Effect(e1));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge1:SetCode(EVENT_CHANGE_POS)
    //     ge1:SetOperation(s.checkop)
    //     Duel.RegisterEffect(ge1,0)
    // end)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_CHANGE_POS);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // for tc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // if tc:IsPreviousPosition(POS_FACEUP_ATTACK) and tc:IsPosition(POS_FACEUP_DEFENSE) then
        if card::is_previous_position(d, tc, POS_FACEUP_ATTACK) && card::is_position(d, tc, POS_FACEUP_DEFENSE) {
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:HasFlagEffect(id) or c:HasFlagEffect(id+1)
    card::has_flag_effect(d, c, ID, None) || card::has_flag_effect(d, c, ID + 1, None)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:HasFlagEffect(id)
    card::is_faceup(d, c) && card::has_flag_effect(d, c, ID, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_OPPO as u64);
    // Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // e1:SetLabel(Duel.GetCurrentChain())
    let chain = duel::get_current_chain(d, false) as i64;
    effect::set_label(d, e1, &[chain]);
    // e1:SetLabelObject(e)
    effect::set_label_object(d, e1, Value::Effect(e));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if re~=e:GetLabelObject() then return end
    let label_effect = effect::get_label_object(d, e).as_effect();
    if a.re != label_effect {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=re:GetLabelObject()
    let re = a.re.nil();
    let tc = effect::get_label_object(d, re).as_card();
    // if tc and c:IsRelateToEffect(re) and tc:IsRelateToEffect(re) and tc:IsFaceup() and Duel.Equip(tp,c,tc) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, re) && card::is_relate_to_effect(d, tc, re) && card::is_faceup(d, tc) && duel::equip(d, tp, c, tc, true, false) {
            // tc:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local dc=g:GetFirst()
    let order = group::iter(d, &g);
    let mut dc = order.first().copied();
    // if dc==tc then dc=g:GetNext() end
    if dc == tc {
        dc = order.get(1).copied();
    }
    // e:SetLabelObject(dc)
    effect::set_label_object(d, e, dc.map_or(Value::Nil, Value::Card));
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, e) && card::is_relate_to_effect(d, tc.nil(), e) && card::is_faceup(d, tc.nil()) {
        // if dc:IsRelateToEffect(e) then
        let dc = dc.nil();
        if card::is_relate_to_effect(d, dc, e) {
            // c:SetCardTarget(dc)
            card::set_card_target(d, c, dc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_OWNER_RELATE)
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_OWNER_RELATE, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
            effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetCondition(s.rcon)
            effect::set_condition(d, e1, Some(condition(rcon)));
            // dc:RegisterEffect(e1,true)
            card::register_effect(d, dc, e1, true);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
            // dc:RegisterEffect(e2,true)
            card::register_effect(d, dc, e2, true);
        }
    }
}
/// `s.rcon(e)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetOwner():IsHasCardTarget(e:GetHandler())
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_LOST_TARGET) and c:GetPreviousEquipTarget():IsReason(REASON_DESTROY)
    //     and c:IsHasCardTarget(e:GetLabelObject():GetLabelObject())
    if !(card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_LOST_TARGET)) {
        return false;
    }
    let prev = card::get_previous_equip_target(d, c).nil();
    if !card::is_reason(d, prev, REASON_DESTROY) {
        return false;
    }
    let act = effect::get_label_object(d, e).as_effect().nil();
    let tc = effect::get_label_object(d, act).as_card().nil();
    card::is_has_card_target(d, c, tc)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=e:GetLabelObject():GetLabelObject()
    let act = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, act).as_card().nil();
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
