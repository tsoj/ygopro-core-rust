//! Inzektor Orb
use super::prelude::*;

pub const ID: u32 = 38643567;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

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
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(cost::remain_field_cost()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INZEKTOR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INZEKTOR as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsLocation(LOCATION_SZONE) or not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.Equip(tp,c,tc)
        duel::equip(d, a.tp, c, tc, true, false);
        // Atk/def
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // e2:SetValue(500)
        effect::set_value(d, e2, 500);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
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
        // negate
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e4, aux::stringid(ID, 0));
        // e4:SetType(EFFECT_TYPE_QUICK_O)
        effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
        // e4:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, e4, CATEGORY_DISABLE);
        // e4:SetCode(EVENT_CHAINING)
        effect::set_code(d, e4, EVENT_CHAINING);
        // e4:SetRange(LOCATION_SZONE)
        effect::set_range(d, e4, LOCATION_SZONE);
        // e4:SetCondition(s.ngcon)
        effect::set_condition(d, e4, Some(condition(ngcon)));
        // e4:SetCost(Cost.SelfToGrave)
        effect::set_cost(d, e4, Some(cost::self_to_grave()));
        // e4:SetTarget(s.ngtg)
        effect::set_target(d, e4, Some(cost_target(ngtg)));
        // e4:SetOperation(s.ngop)
        effect::set_operation(d, e4, Some(operation(ngop)));
        // e4:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e4)
        card::register_effect(d, c, e4, false);
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_INZEKTOR)
    card::is_set_card(d, c, &[SET_INZEKTOR as u16])
}
/// `s.ngcon(e,tp,eg,ep,ev,re,r,rp)`
fn ngcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local loc,tg=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TARGET_CARDS]);
    let loc = info[0].int();
    let tg = info[1].as_group().cloned().nil();
    // local tc=tg:GetFirst()
    let tc = group::get_first(d, &tg);
    // if #tg~=1 or not tc:IsLocation(LOCATION_MZONE) or not tc:IsSetCard(SET_INZEKTOR) then return false end
    if tg.len() != 1 {
        return false;
    }
    let tc = tc.nil();
    if !card::is_location(d, tc, LOCATION_MZONE) || !card::is_set_card(d, tc, &[SET_INZEKTOR as u16]) {
        return false;
    }
    // return Duel.IsChainDisablable(ev) and loc~=LOCATION_DECK
    duel::is_chain_disablable(d, a.ev as u8) && loc != LOCATION_DECK as i64
}
/// `s.ngtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ngtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.ngop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ngop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
