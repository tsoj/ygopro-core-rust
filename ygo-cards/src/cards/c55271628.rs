//! Tyrant's Throes
use super::prelude::*;

pub const ID: u32 = 55271628;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot Normal or Special Summon Effect Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(s.sumlimit)
    effect::set_target(d, e2, Some(effect_card_fn(sumlimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and not c:IsType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_NORMAL) && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroup(tp,s.cfilter,2,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group(d, a.tp, Some(&cfilter), 2, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local rg=Duel.SelectReleaseGroup(tp,s.cfilter,2,2,nil)
    let rg = duel::select_release_group(d, a.tp, Some(&cfilter), 2, 2, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return c:IsType(TYPE_EFFECT)
        card::is_type(d, c, TYPE_EFFECT)
    } else {
        // return c:IsOriginalType(TYPE_EFFECT)
        card::is_original_type(d, c, TYPE_EFFECT)
    }
}
