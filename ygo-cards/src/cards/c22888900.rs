//! Grisaille Prison
use super::prelude::*;

pub const ID: u32 = 22888900;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsTributeSummoned() or c:IsRitualSummoned() or c:IsFusionSummoned())
    card::is_faceup(d, c)
        && (card::is_tribute_summoned(d, c) || card::is_ritual_summoned(d, c) || card::is_fusion_summoned(d, c))
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO+TYPE_XYZ)
    card::is_type(d, c, TYPE_SYNCHRO | TYPE_XYZ)
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    let sumtp = a.int(3);
    // return (sumtp&SUMMON_TYPE_SYNCHRO)==SUMMON_TYPE_SYNCHRO or (sumtp&SUMMON_TYPE_XYZ)==SUMMON_TYPE_XYZ
    (sumtp & SUMMON_TYPE_SYNCHRO as i64) == SUMMON_TYPE_SYNCHRO as i64
        || (sumtp & SUMMON_TYPE_XYZ as i64) == SUMMON_TYPE_XYZ as i64
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(effect_card_fn(tg)));
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // if Duel.IsTurnPlayer(1-tp) then
    if duel::is_turn_player(d, 1 - tp) {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetTarget(s.splimit)
    effect::set_target(d, e3, Some(lua_fn(splimit)));
    // if Duel.IsTurnPlayer(1-tp) then
    if duel::is_turn_player(d, 1 - tp) {
        // e3:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
    } else {
        // e3:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    }
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
