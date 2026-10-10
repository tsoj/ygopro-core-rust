//! Rare Metalmorph
use super::prelude::*;

pub const ID: u32 = 12503902;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip procedure
    // aux.AddPersistentProcedure(c,0,s.filter,CATEGORY_ATKCHANGE,EFFECT_FLAG_DAMAGE_STEP,nil,TIMING_DAMAGE_STEP,aux.StatChangeDamageStepCondition,nil,nil,s.operation)
    aux::add_persistent_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        Some(CATEGORY_ATKCHANGE),
        Some(EFFECT_FLAG_DAMAGE_STEP),
        None,
        Some(TIMING_DAMAGE_STEP),
        Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))),
        None,
        None,
        Some(operation(op)),
        false,
    );
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy this card when the monster leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(s.selfdescond)
    effect::set_condition(d, e2, Some(condition(selfdescond)));
    // e2:SetOperation(s.selfdesop)
    effect::set_operation(d, e2, Some(operation(selfdesop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and s.filter(tc) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && filter(d, tc.nil()) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // e1:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
        // e1:SetCondition(s.discon)
        effect::set_condition(d, e1, Some(condition(discon)));
        // e1:SetOperation(s.disop)
        effect::set_operation(d, e1, Some(operation(disop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, c);
    // if not tc or not re:IsSpellEffect() or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let Some(tc) = tc else { return false };
    let re = a.re.nil();
    if !effect::is_spell_effect(d, re) || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // return g:IsContains(tc)
    g.contains(tc)
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
    // e:Reset()
    effect::reset(d, a.e);
}
// function s.selfdescond(e,tp,eg,ep,ev,re,r,rp)
fn selfdescond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    // return tc and eg:IsContains(tc)
    match card::get_first_card_target(d, c) {
        Some(tc) => a.eg.clone().nil().contains(tc),
        None => false,
    }
}
// function s.selfdesop(e,tp,eg,ep,ev,re,r,rp)
fn selfdesop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
