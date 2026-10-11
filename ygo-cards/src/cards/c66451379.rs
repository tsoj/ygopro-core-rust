//! Graydle Alligator
use super::prelude::*;

pub const ID: u32 = 66451379;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.eqcon(e,tp,eg,ep,ev,re,r,rp)
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and re and re:IsSpellEffect()))
    let by_battle = card::is_reason(d, c, REASON_BATTLE);
    let by_spell = !by_battle
        && card::is_reason(d, c, REASON_EFFECT)
        && match a.re {
            Some(re) => effect::is_spell_effect(d, re),
            None => false,
        };
    (by_battle || by_spell)
        // and c:IsReason(REASON_DESTROY) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and aux.CheckStealEquip(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && aux::check_steal_equip(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.CheckStealEquip,tp,0,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| aux::check_steal_equip(d, c, a.e, a.tp)),
            a.tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,aux.CheckStealEquip,tp,0,LOCATION_MZONE,1,1,nil,e,tp)
    let _ = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| aux::check_steal_equip(d, c, a.e, a.tp)),
        a.tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.eqlimit(e,c) return c==e:GetLabelObject() end
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and aux.CheckStealEquip(tc,e,tp) and Duel.Equip(tp,c,tc,true) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e)
            && aux::check_steal_equip(d, tc, a.e, tp)
            && duel::equip(d, tp, c, tc, true, false)
        {
            // Add Equip limit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(s.eqlimit)
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            // e1:SetLabelObject(tc)
            effect::set_label_object(d, e1, Value::Card(tc));
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // control
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            // e2:SetCode(EFFECT_SET_CONTROL)
            effect::set_code(d, e2, EFFECT_SET_CONTROL);
            // e2:SetValue(tp)
            effect::set_value(d, e2, tp as i64);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // Destroy
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
            // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e3:SetCode(EVENT_LEAVE_FIELD_P)
            effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
            // e3:SetOperation(s.checkop)
            effect::set_operation(d, e3, Some(operation(checkop)));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
            // local e4=Effect.CreateEffect(c)
            let e4 = effect::create_effect(d, c);
            // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
            effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
            // e4:SetCode(EVENT_LEAVE_FIELD)
            effect::set_code(d, e4, EVENT_LEAVE_FIELD);
            // e4:SetOperation(s.desop)
            effect::set_operation(d, e4, Some(operation(desop)));
            // e4:SetReset(RESET_EVENT|RESET_OVERLAY|RESET_TOFIELD)
            effect::set_reset(d, e4, RESET_EVENT | RESET_OVERLAY | RESET_TOFIELD, 1);
            // e4:SetLabelObject(e3)
            effect::set_label_object(d, e4, Value::Effect(e3));
            // c:RegisterEffect(e4)
            card::register_effect(d, c, e4, false);
        }
    }
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsDisabled() then e:SetLabel(1) else e:SetLabel(0) end
    let c = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, c) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // e:Reset()
    effect::reset(d, a.e);
    // if e:GetLabelObject():GetLabel()~=0 then return end
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) != 0 {
        return;
    }
    // local tc=e:GetHandler():GetEquipTarget()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsLocation(LOCATION_MZONE) then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = card::get_equip_target(d, c) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
