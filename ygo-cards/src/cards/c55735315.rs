//! Soul Drain Dragon
use super::prelude::*;

pub const ID: u32 = 55735315;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.splimit(e,se,sp,st)`: `sc:IsType(TYPE_XYZ) and sc:IsRace(RACE_DRAGON)` for the summoning effect's card
fn splimit(d: &mut Duel, a: Args) -> bool {
    // local sc=se:GetHandler()
    let sc = effect::get_handler(d, a.e(1)).nil();
    card::is_type(d, sc, TYPE_XYZ) && card::is_race(d, sc, RACE_DRAGON)
}

/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`: `Duel.GetLP(tp)<Duel.GetLP(1-tp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, a.tp) < duel::get_lp(d, 1 - a.tp)
}

/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) and Duel.GetLP(tp)<Duel.GetLP(1-tp) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && duel::get_lp(d, tp) < duel::get_lp(d, 1 - tp) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(Duel.GetLP(1-tp)-Duel.GetLP(tp))
        let diff = duel::get_lp(d, 1 - tp) - duel::get_lp(d, tp);
        effect::set_value(d, e1, diff as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(0)
    effect::set_value(d, e2, 0);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e3, EFFECT_NO_EFFECT_DAMAGE);
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
