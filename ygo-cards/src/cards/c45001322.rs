//! Libromancer Firestarter
//! リブロマンサー・ファイアスターター
use super::prelude::*;

pub const ID: u32 = 45001322;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIBROMANCER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Check materials use for its Ritual Summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.matcheck)
    effect::set_value_fn(d, e0, lua_fn(matcheck));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.matcon)
    effect::set_condition(d, e1, Some(condition(matcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be banished by card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e2, EFFECT_CANNOT_REMOVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(function(e,c,tp,r) return c==e:GetHandler() and r==REASON_EFFECT end)
    effect::set_target(d, e2, Some(lua_fn(|d, a: Args| {
        let c = a.c(1);
        let h = effect::get_handler(d, a.e(0)).nil();
        c == h && a.int(3) == REASON_EFFECT as i64
    })));
    // e2:SetCondition(s.matcon)
    effect::set_condition(d, e2, Some(condition(matcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register a flag when an effect resolves
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetOperation(s.chainop)
    effect::set_operation(d, e3, Some(operation(chainop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Gains 200 ATK/DEF each time an effect activated by the opponent resolves
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e4, EVENT_CHAIN_SOLVED);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    // if c:GetMaterial():IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) then
    let mat = card::get_material(d, c);
    if group::is_exists(d, &mat, &|d, cc| card::is_location(d, cc, LOCATION_MZONE), 1, NO_EX) {
        // local reset=RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD
        let reset = RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD);
        // c:RegisterFlagEffect(id,reset,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
        card::register_flag_effect(d, c, ID, reset, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 0));
    }
    Value::Nil
}
/// `s.matcon(e)`
fn matcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRitualSummoned() and c:HasFlagEffect(id)
    card::is_ritual_summoned(d, c) && card::has_flag_effect(d, c, ID, None)
}
/// `s.chainop(e,tp,eg,ep,ev,re,r,rp)`
fn chainop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN|RESET_TURN_SET,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN | RESET_TURN_SET, 0, 1, 0, 0);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return ep==1-tp and c:GetAttack()<3000 and c:HasFlagEffect(id+1)
    a.ep == 1 - a.tp && card::get_attack(d, c) < 3000 && card::has_flag_effect(d, c, ID + 1, None)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --Gain 200 ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
    effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
    // e1:SetValue(200)
    effect::set_value(d, e1, 200);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
