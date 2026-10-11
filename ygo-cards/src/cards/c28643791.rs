//! March Towards Ragnarok
//! 最後の進軍
use super::prelude::*;

pub const ID: u32 = 28643791;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AESIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted "Aesir" monster has its effects negated, also unaffected by spells/traps
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
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
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AESIR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AESIR as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
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
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let _g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc.nil()) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Negate its effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c.nil());
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c.nil());
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc.nil(), e2, false);
        // Unaffected by spells/traps
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c.nil());
        // e3:SetDescription(3104)
        effect::set_description(d, e3, 3104);
        // e3:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
        // e3:SetValue(s.imfilter)
        effect::set_value_fn(d, e3, lua_fn(|d, a: Args| imfilter(d, a.e(0), a.e(1))));
        // e3:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc.nil(), e3, false);
    }
}
/// `s.imfilter(e,re)`
fn imfilter(d: &mut Duel, e: EffectId, re: EffectId) -> bool {
    // return re:IsSpellTrapEffect() and re:GetOwner()~=e:GetOwner()
    effect::is_spell_trap_effect(d, re) && effect::get_owner(d, re) != effect::get_owner(d, e)
}
