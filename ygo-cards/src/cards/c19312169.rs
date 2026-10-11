//! Talisman of Trap Sealing
use super::prelude::*;

pub const ID: u32 = 19312169;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[2468169], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.actcon)
    effect::set_condition(d, e1, Some(condition(actcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e2, EFFECT_SELF_DESTROY);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent activation
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_HAND|LOCATION_SZONE,LOCATION_HAND|LOCATION_SZONE)
    effect::set_target_range(d, e3, LOCATION_HAND | LOCATION_SZONE, LOCATION_HAND | LOCATION_SZONE);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsTrap))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate traps
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e4, EFFECT_DISABLE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e4, LOCATION_SZONE, LOCATION_SZONE);
    // e4:SetTarget(aux.TargetBoolFunction(Card.IsTrap))
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Disable effect
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e5, EVENT_CHAIN_SOLVING);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetOperation(s.disop)
    effect::set_operation(d, e5, Some(operation(disop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Disable trap monster
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetCode(EFFECT_DISABLE_TRAPMONSTER)
    effect::set_code(d, e6, EFFECT_DISABLE_TRAPMONSTER);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e6, LOCATION_MZONE, LOCATION_MZONE);
    // e6:SetTarget(aux.TargetBoolFunction(Card.IsTrap))
    effect::set_target(d, e6, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // aux.DoubleSnareValidity(c,LOCATION_SZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
/// `s.actcon(e,tp,eg,ep,ev,re,r,rp)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,2468169),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[2468169])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.descon(e)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,2468169),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[2468169])), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // if tl==LOCATION_SZONE and re:IsTrapEffect() then
    if tl == LOCATION_SZONE as i64 && effect::is_trap_effect(d, a.re.nil()) {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
