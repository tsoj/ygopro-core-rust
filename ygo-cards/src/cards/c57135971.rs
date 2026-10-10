//! Red-Eyes Fang with Chain
use super::prelude::*;

pub const ID: u32 = 57135971;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RED_EYES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // sand to grave and equip
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RED_EYES) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_RED_EYES as u16]) && card::is_faceup(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return s.filter(chkc) and chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return filter(d, chkc) && card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_relate_to_effect(d, c, e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc.filter(|&tc| card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        // Duel.Equip(tp,c,tc)
        duel::equip(d, tp, c, tc, true, false);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Equip limit
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e3, EFFECT_EQUIP_LIMIT);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetValue(s.eqlimit)
        effect::set_value_fn(d, e3, effect_card_fn(eqlimit));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_RED_EYES)
    card::is_set_card(d, c, &[SET_RED_EYES as u16])
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget() and aux.StatChangeDamageStepCondition()
    let c = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, c).is_some() && aux::stat_change_damage_step_condition(d)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c);
    }
    // e:SetLabelObject(e:GetHandler():GetEquipTarget())
    let eq = match card::get_equip_target(d, c) {
        Some(eq) => Value::Card(eq),
        None => Value::Nil,
    };
    effect::set_label_object(d, e, eq);
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `Exclude` for an optional card (`nil` excludes nothing)
fn ex_of<'a>(c: Option<CardId>) -> Exclude<'a> {
    match c {
        Some(c) => c.into(),
        None => NO_EX,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, e);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.desfilter(chkc) and chkc~=ec end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && desfilter(d, chkc) && ec.as_card() != Some(chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler():GetEquipTarget()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        let eq = card::get_equip_target(d, h);
        return duel::is_existing_target(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, ex_of(eq));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,ec)
    let g = duel::select_target(d, tp, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, ex_of(ec.as_card()));
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.eqlimit2(e,c)`: `c==e:GetLabelObject()`
fn eqlimit2(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, e);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // if ec and ec:IsLocation(LOCATION_MZONE) and ec:IsFaceup() then
    let equip_card = match ec.as_card() {
        Some(ecc) if card::is_location(d, ecc, LOCATION_MZONE) && card::is_faceup(d, ecc) => Some(ecc),
        _ => None,
    };
    if let Some(ecc) = equip_card {
        // Duel.Equip(tp,tc,ec)
        duel::equip(d, tp, tc, ecc, true, false);
        // local e1=Effect.CreateEffect(tc)
        let e1 = effect::create_effect(d, tc);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_SET_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(tc:GetAttack())
        let atk = card::get_attack(d, tc) as i64;
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if not tc:IsOriginalType(TYPE_LINK) then
        if !card::is_original_type(d, tc, TYPE_LINK) {
            // local e2=Effect.CreateEffect(tc)
            let e2 = effect::create_effect(d, tc);
            // e2:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            // e2:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE);
            // e2:SetValue(tc:GetDefense())
            let def = card::get_defense(d, tc) as i64;
            effect::set_value(d, e2, def);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
        // Equip limit
        // local e3=Effect.CreateEffect(tc)
        let e3 = effect::create_effect(d, tc);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e3, EFFECT_EQUIP_LIMIT);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetValue(s.eqlimit2)
        effect::set_value_fn(d, e3, effect_card_fn(eqlimit2));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // e3:SetLabelObject(ec)
        effect::set_label_object(d, e3, ec);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    } else {
        // Duel.SendtoGrave(tc,REASON_RULE)
        duel::send_to_grave(d, tc, REASON_RULE, None, None);
    }
}
