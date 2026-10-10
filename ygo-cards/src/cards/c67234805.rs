//! Powersink Stone
use super::prelude::*;

pub const ID: u32 = 67234805;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[0x16],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x16)
    card::enable_counter_permit(d, c, 0x16, None, None);
    // c:SetCounterLimit(0x16,2)
    card::set_counter_limit(d, c, 0x16, 2);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 Spellstone Counter each time a monster effect is activated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(aux.chainreg)
    effect::set_operation(d, e2, Some(aux::chainreg()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e2=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2b, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2b, EVENT_CHAIN_SOLVING);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2b, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2b, LOCATION_SZONE);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2b, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2b, false);
    // Monsters on the field cannot activate their effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():GetCounter(0x16)==2 end)
    effect::set_condition(d, e3, Some(condition(counter_is_two)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate the effects of all monsters on the field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e4, EFFECT_DISABLE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetCondition(function(e) return e:GetHandler():GetCounter(0x16)==2 end)
    effect::set_condition(d, e4, Some(condition(counter_is_two)));
    // e4:SetTarget(function(e,c) return c:IsType(TYPE_EFFECT) end)
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_EFFECT))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Remove all Spellstone Counters from this card during the End Phase
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(function(e) return e:GetHandler():GetCounter(0x16)>0 end)
    effect::set_condition(d, e5, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_counter(d, h, 0x16) > 0
    })));
    // e5:SetOperation(s.rmop)
    effect::set_operation(d, e5, Some(operation(rmop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `function(e) return e:GetHandler():GetCounter(0x16)==2 end`
fn counter_is_two(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0x16) == 2
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if re:IsMonsterEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    if effect::is_monster_effect(d, a.re.nil()) {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, 1) > 0 {
            // e:GetHandler():AddCounter(0x16,1)
            card::add_counter(d, h, 0x16, 1, false);
        }
    }
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(tp,0x16,e:GetHandler():GetCounter(0x16),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    let n = card::get_counter(d, h, 0x16);
    card::remove_counter(d, h, a.tp, 0x16, n as u16, REASON_EFFECT);
}
