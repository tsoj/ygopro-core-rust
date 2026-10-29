//! Yamatako Orochi
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 4632019;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be treated as Level 8 for a Synchro Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SYNCHRO_LEVEL)
    effect::set_code(d, e1, EFFECT_SYNCHRO_LEVEL);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_ONFIELD)
    effect::set_range(d, e1, LOCATION_ONFIELD);
    // e1:SetValue(function(e) return 8*65536+e:GetHandler():GetLevel() end)
    effect::set_value_fn(d, e1, effect_value(synchro_level));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Synchro Monster gains effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The value function of e1: `8*65536+e:GetHandler():GetLevel()`
fn synchro_level(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    (8i64 * 65536) + card::get_level(d, h) as i64
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return r==REASON_SYNCHRO and e:GetHandler():GetReasonCard():HasLevel()
    if a.r != REASON_SYNCHRO {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, h).nil();
    card::has_level(d, rc)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // if rc:GetOriginalLevel()<=8 then
    if card::get_original_level(d, rc) <= 8 {
        // Gain 800 ATK/DEF
        // local e1=Effect.CreateEffect(rc)
        let e1 = effect::create_effect(d, rc);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(800)
        effect::set_value(d, e1, 800);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e1)
        card::register_effect(d, rc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // rc:RegisterEffect(e2)
        card::register_effect(d, rc, e2, false);
    } else {
        // Inflict piercing damage
        // local e1=Effect.CreateEffect(rc)
        let e1 = effect::create_effect(d, rc);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e1, EFFECT_PIERCE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e1)
        card::register_effect(d, rc, e1, false);
    }
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e3, EFFECT_ADD_TYPE);
        // e3:SetValue(TYPE_EFFECT)
        effect::set_value(d, e3, TYPE_EFFECT as i64);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e3,true)
        card::register_effect(d, rc, e3, true);
    }
    // rc:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
    card::register_flag_effect(d, rc, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 0));
}
