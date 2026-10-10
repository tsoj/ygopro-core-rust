//! Fusion Tag
use super::prelude::*;

pub const ID: u32 = 59432181;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_EXTRA,0,1,nil,c,tp)
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, c, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.cfilter(c,tc,tp)`
fn cfilter(d: &mut Duel, c: CardId, tc: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION) and not c:IsCode(tc:GetCode(nil,SUMMON_TYPE_FUSION,tp))
    let codes = card::get_code_ctx(d, tc, None, SUMMON_TYPE_FUSION as u64, Some(tp));
    card::is_type(d, c, TYPE_FUSION) && !card::is_code(d, c, &codes)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let _g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_EXTRA,0,1,1,nil,tc,tp)
    let cg = duel::select_matching_card(d, tp, Some(&|d, x| cfilter(d, x, tc, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #cg==0 then return end
    if cg.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - tp, cg.clone());
    // local code1,code2=cg:GetFirst():GetOriginalCodeRule()
    let first = group::get_first(d, &cg).nil();
    let codes = card::get_original_code_rule(d, first);
    let code1 = codes[0];
    let code2 = codes.get(1).copied();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ADD_CODE)
    effect::set_code(d, e1, EFFECT_ADD_CODE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(code1)
    effect::set_value(d, e1, code1 as i64);
    // e1:SetOperation(s.chngcon)
    effect::set_operation(d, e1, Some(condition(chngcon)));
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // if code2 then
    if let Some(code2) = code2 {
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetValue(code2)
        effect::set_value(d, e2, code2 as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.chngcon(scard,sumtype,tp)`: the core never runs this operation (the effect is a value
/// effect); the arguments are read by position as the Lua function would get them from the call.
fn chngcon(d: &mut Duel, a: &EffArgs) -> bool {
    let sumtype = a.tp as u64;
    let _ = d;
    // return (sumtype&MATERIAL_FUSION)~=0 or (sumtype&SUMMON_TYPE_FUSION)~=0
    (sumtype & MATERIAL_FUSION as u64) != 0 || (sumtype & SUMMON_TYPE_FUSION as u64) != 0
}
