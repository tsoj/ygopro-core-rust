//! Karakuri Barrel mdl 96 "Shinkuro"
use super::prelude::*;

pub const ID: u32 = 92300891;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // must attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to defense
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e3:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    // e3:SetCondition(s.poscon)
    effect::set_condition(d, e3, Some(condition(poscon)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // battle indes
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_COUNT);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetValue(s.valcon)
    effect::set_value_fn(d, e4, lua_fn(valcon));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}

/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`: `e:GetHandler():IsAttackPos()`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}

/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}

/// `s.valcon(e,re,r,rp)`: `(r&REASON_BATTLE)~=0`
fn valcon(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
