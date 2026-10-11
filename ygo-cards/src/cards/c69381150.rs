//! A.I.Q
use super::prelude::*;

pub const ID: u32 = 69381150;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_IGNISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Limited Link Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Maintain
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_IGNISTER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_IGNISTER as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonType,1,nil,SUMMON_TYPE_LINK)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_LINK]), 1, NO_EX)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local players=eg:Filter(Card.IsSummonType,nil,SUMMON_TYPE_LINK):GetClass(Card.GetSummonPlayer)
    let eg = a.eg.clone().nil();
    let linked = group::filter(d, &eg, &|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_LINK]), NO_EX);
    let players = group::get_class(d, &linked, &|d, c| card::get_summon_player(d, c) as i64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for _,p in ipairs(players) do
    for p in players {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetAbsoluteRange(0,1-p,p)
        effect::set_absolute_range(d, e1, 0, (1 - p) as u32, p as u32);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // e1:SetTarget(s.splimit)
        effect::set_target(d, e1, Some(lua_fn(splimit)));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    // return sumtype&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK
    let sumtype = a.int(3) as u64;
    (sumtype & SUMMON_TYPE_LINK as u64) == SUMMON_TYPE_LINK as u64
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReleasableByEffect() and c:IsType(TYPE_LINK)
    card::is_releasable_by_effect(d, c, None, None) && card::is_type(d, c, TYPE_LINK)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, CardSet::from_card(c), true);
    // if Duel.CheckReleaseGroup(tp,s.desfilter,1,nil) and Duel.SelectEffectYesNo(tp,c) then
    if duel::check_release_group(d, tp, Some(&desfilter), 1, duel::ReleaseArgs::default(), NO_EX, false)
        && duel::select_effect_yes_no(d, tp, c, None)
    {
        // local g=Duel.SelectReleaseGroup(tp,s.desfilter,1,1,nil)
        let g = duel::select_release_group(d, tp, Some(&desfilter), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    } else {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, CardSet::from_card(c), REASON_COST, None, None);
    }
}
