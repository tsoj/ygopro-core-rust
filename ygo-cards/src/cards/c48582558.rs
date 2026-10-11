//! Bounzer Guard
use super::prelude::*;

pub const ID: u32 = 48582558;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BOUNZER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted "Bounzer" monster cannot be targeted by card effect, also cannot be destroyed by battle
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMING_BATTLE_START) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BOUNZER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BOUNZER as u16])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            // e1:SetDescription(3000)
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3000);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetDescription(3002)
            effect::set_description(d, e2, 3002);
            // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
            effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, h);
            // e3:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD);
            // e3:SetCode(EFFECT_MUST_ATTACK_MONSTER)
            effect::set_code(d, e3, EFFECT_MUST_ATTACK_MONSTER);
            // e3:SetTargetRange(0,LOCATION_MZONE)
            effect::set_target_range(d, e3, 0, LOCATION_MZONE);
            // e3:SetValue(s.atklimit)
            effect::set_value_fn(d, e3, effect_card_fn(atklimit));
            // e3:SetLabel(tc:GetRealFieldID())
            let rfid = card::get_real_field_id(d, tc);
            effect::set_label(d, e3, &[rfid as i64]);
            // e3:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e3,tp)
            duel::register_effect(d, e3, a.tp);
        }
    }
}
// function s.atklimit(e,c)
fn atklimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetRealFieldID()==e:GetLabel()
    card::get_real_field_id(d, c) as i64 == effect::get_label(d, e)
}
