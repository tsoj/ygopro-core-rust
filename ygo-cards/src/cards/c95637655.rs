//! Backup Warrior
use super::prelude::*;

pub const ID: u32 = 95637655;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Check/apply Synchro Summon restriction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e2, EFFECT_SPSUMMON_COST);
    // e2:SetCost(function(_,_,tp) return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end)
    effect::set_cost(d, e2, Some(lua_fn(spcost_check)));
    // e2:SetOperation(s.spcostop)
    effect::set_operation(d, e2, Some(operation(spcostop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not c:IsSynchroSummoned() end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counter_filter));
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and not Duel.IsExistingMatchingCard(Card.IsAttackPos,tp,LOCATION_MZONE,0,1,nil)
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 2
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_attack_pos(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `function(_,_,tp) return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end`
fn spcost_check(d: &mut Duel, a: Args) -> bool {
    duel::get_custom_activity_count(d, ID, a.p(2), ACTIVITY_SPSUMMON) == 0
}
/// `function(c) return not c:IsSynchroSummoned() end` (the activity counter filter)
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    !card::is_synchro_summoned(d, a.c(0))
}
/// `s.spcostop(e,tp,eg,ep,ev,re,r,rp)`
fn spcostop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,_,_,sumtype) return (sumtype&SUMMON_TYPE_SYNCHRO)==SUMMON_TYPE_SYNCHRO end)
    effect::set_target(d, e1, Some(lua_fn(synchro_limit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `function(_,_,_,sumtype) return (sumtype&SUMMON_TYPE_SYNCHRO)==SUMMON_TYPE_SYNCHRO end`
fn synchro_limit(_d: &mut Duel, a: Args) -> bool {
    let sumtype = a.int(3);
    (sumtype & SUMMON_TYPE_SYNCHRO as i64) == SUMMON_TYPE_SYNCHRO as i64
}
