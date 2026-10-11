//! Zekt Conversion
use super::prelude::*;

pub const ID: u32 = 74854609;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d).nil();
    // return d:IsFaceup() and d:IsSetCard(SET_INZEKTOR) and a:IsControler(1-tp)
    card::is_faceup(d, dd) && card::is_set_card(d, dd, &[SET_INZEKTOR as u16]) && card::is_controler(d, att, 1 - a.tp)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // if chkc then return a==chkc end
    if let Some(chkc) = a.chkc() {
        return att == Some(chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and a:IsOnField() and a:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let att = att.nil();
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && card::is_on_field(d, att)
            && card::is_can_be_effect_target(d, att, Some(a.e));
    }
    // Duel.SetTargetCard(a)
    duel::set_target_card(d, att.nil());
    true
}
// s.eqlimit(e,c)
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
// s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=Duel.GetAttackTarget()
    let ec = duel::get_attack_target(d);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:CanAttack() and not tc:IsStatus(STATUS_ATTACK_CANCELED)
    //   and ec:IsLocation(LOCATION_MZONE) and ec:IsFaceup() then
    let Some(tc) = tc else { return };
    if !(card::can_attack(d, tc) && !card::is_status(d, tc, STATUS_ATTACK_CANCELED)) {
        return;
    }
    let ec = ec.nil();
    if !(card::is_location(d, ec, LOCATION_MZONE) && card::is_faceup(d, ec)) {
        return;
    }
    // if not Duel.Equip(tp,ec,tc) then return end
    if !duel::equip(d, a.tp, ec, tc, true, false) {
        return;
    }
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
    // ec:RegisterEffect(e1)
    card::register_effect(d, ec, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_SET_CONTROL)
    effect::set_code(d, e2, EFFECT_SET_CONTROL);
    // e2:SetValue(tp)
    effect::set_value(d, e2, a.tp as i64);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // ec:RegisterEffect(e2)
    card::register_effect(d, ec, e2, false);
}
