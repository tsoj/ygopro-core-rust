//! Amorphage Goliath
use super::prelude::*;

pub const ID: u32 = 69072185;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMORPHAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Flag any "Amorphage" monster that would leave your field to exclude them in the banish effect's condition
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e0, EVENT_LEAVE_FIELD_P);
    // e0:SetRange(LOCATION_PZONE)
    effect::set_range(d, e0, LOCATION_PZONE);
    // e0:SetOperation(s.amorphreg)
    effect::set_operation(d, e0, Some(operation(amorphreg)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Any card sent to the GY is banished instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetTarget(function(e,_c) return not _c:IsOriginalSetCard(SET_AMORPHAGE) and Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),_c) end)
    effect::set_target(d, e1, Some(effect_card_fn(redirect_target)));
    // e1:SetTargetRange(LOCATION_ALL,LOCATION_ALL)
    effect::set_target_range(d, e1, LOCATION_ALL, LOCATION_ALL);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Maintenance cost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(maint_condition)));
    // e2:SetOperation(s.maintop)
    effect::set_operation(d, e2, Some(operation(maintop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Neither player can Special Summon monsters from the Extra Deck, except "Amorphage" monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetTarget(function(e,_c) return _c:IsLocation(LOCATION_EXTRA) and not _c:IsSetCard(SET_AMORPHAGE) end)
    effect::set_target(d, e3, Some(effect_card_fn(extra_target)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Clock Lizard check
    // aux.addContinuousLizardCheck(c,LOCATION_MZONE,function(e,_c) return not _c:IsOriginalSetCard(SET_AMORPHAGE) end,LOCATION_ALL,LOCATION_ALL)
    aux::add_continuous_lizard_check(d, c, LOCATION_MZONE, Some(effect_card_fn(lizard_filter)), Some(LOCATION_ALL), Some(LOCATION_ALL));
}
/// `function(e,_c) return not _c:IsOriginalSetCard(SET_AMORPHAGE) and Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),_c) end`
fn redirect_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    !card::is_original_set_card(d, c, &[SET_AMORPHAGE as u16]) && {
        let p = effect::get_handler_player(d, e);
        duel::is_player_can_remove(d, p, Some((c, None)))
    }
}
/// `function(e,_c) return _c:IsLocation(LOCATION_EXTRA) and not _c:IsSetCard(SET_AMORPHAGE) end`
fn extra_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_set_card(d, c, &[SET_AMORPHAGE as u16])
}
/// `function(e,_c) return not _c:IsOriginalSetCard(SET_AMORPHAGE) end` (the Clock Lizard check)
fn lizard_filter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_original_set_card(d, c, &[SET_AMORPHAGE as u16])
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn maint_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.amorphreg(e,tp,eg,ep,ev,re,r,rp)`
fn amorphreg(d: &mut Duel, a: &EffArgs) {
    // for tc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // if tc:IsSetCard(SET_AMORPHAGE) and tc:IsLocation(LOCATION_MZONE) and tc:IsFaceup()
        //     and tc:IsControler(tp) then
        if card::is_set_card(d, tc, &[SET_AMORPHAGE as u16]) && card::is_location(d, tc, LOCATION_MZONE) && card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.rmconfilter(c)`
fn rmconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_AMORPHAGE) and c:IsFaceup() and not c:HasFlagEffect(id)
    card::is_set_card(d, c, &[SET_AMORPHAGE as u16]) && card::is_faceup(d, c) && !card::has_flag_effect(d, c, ID, None)
}
/// `s.rmcon(e)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.rmconfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&rmconfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.maintop(e,tp,eg,ep,ev,re,r,rp)`
fn maintop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local b1=Duel.CheckReleaseGroup(tp,Card.IsReleasableByEffect,1,c)
    let b1 = duel::check_release_group(d, tp, Some(&|d, x| card::is_releasable_by_effect(d, x, None, None)), 1, duel::ReleaseArgs::default(), c.into(), false);
    // local b2=true
    let b2 = true;
    // local op=b1 and Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,0)},
    //     {b2,aux.Stringid(id,1)}) or 2
    let op = if b1 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]).unwrap_or(2)
    } else {
        2
    };
    // if op==1 then
    if op == 1 {
        // local g=Duel.SelectReleaseGroup(tp,Card.IsReleasableByEffect,1,1,c)
        let g = duel::select_release_group(d, tp, Some(&|d, x| card::is_releasable_by_effect(d, x, None, None)), 1, 1, duel::ReleaseArgs::default(), c.into(), false).nil();
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
