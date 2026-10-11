//! チューン・ナイト (Squeaknight)
use super::prelude::*;

pub const ID: u32 = 38783169;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // tuner
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetOperation(s.tuneop)
    effect::set_operation(d, e1, Some(operation(tuneop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // union
    // aux.AddUnionProcedure(c)
    aux::add_union_procedure(d, c, None, false, None);
}
/// `s.tuneop(e,tp,...)`
fn tuneop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e0=aux.createTempLizardCheck(c)
    let e0 = aux::create_temp_lizard_check(d, c, None, None, None, None, None);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(condition(spcon)));
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e2, a.tp);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsLocation,LOCATION_EXTRA))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // lizard check with a reset
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetOperation(s.checkop)
    effect::set_operation(d, e3, Some(operation(checkop)));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
/// `s.spcon(e)`: `return Duel.GetFlagEffect(e:GetOwnerPlayer(),id)>0`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_owner_player(d, a.e);
    duel::get_flag_effect(d, p, ID) > 0
}
/// `s.checkop(e,tp,eg,...)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(Card.IsPreviousLocation,1,nil,LOCATION_EXTRA) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_EXTRA), 1, NO_EX) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
