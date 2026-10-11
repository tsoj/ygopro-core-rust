//! Magnet Force
use super::prelude::*;

pub const ID: u32 = 17841166;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.etarget)
    effect::set_target(d, e1, Some(effect_card_fn(etarget)));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.etarget(e,c)`
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetOriginalRace()&(RACE_MACHINE|RACE_ROCK)~=0
    (card::get_original_race(d, c) & (RACE_MACHINE | RACE_ROCK)) != 0
}
/// `s.efilter(e,te,c)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, te, c) = (a.e(0), a.e(1), a.c(2));
    // return te:IsMonsterEffect() and te:GetOwner()~=c and te:GetOwnerPlayer()~=e:GetHandlerPlayer()
    effect::is_monster_effect(d, te)
        && effect::get_owner(d, te) != Some(c)
        && effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
}
