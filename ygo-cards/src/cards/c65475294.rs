//! The Unfriendly Amazon
use super::prelude::*;

pub const ID: u32 = 65475294;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cost
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.costcon)
    effect::set_condition(d, e1, Some(condition(costcon)));
    // e1:SetOperation(s.costop)
    effect::set_operation(d, e1, Some(operation(costop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costcon(e,tp,eg,ep,ev,re,r,rp)`
fn costcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.costop(e,tp,eg,ep,ev,re,r,rp)`
fn costop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.CheckReleaseGroup(tp,nil,1,c) then
    if duel::check_release_group(d, a.tp, None, 1, duel::ReleaseArgs::default(), c.into(), false) {
        // local g=Duel.SelectReleaseGroup(tp,nil,1,1,c)
        let g = duel::select_release_group(d, a.tp, None, 1, 1, duel::ReleaseArgs::default(), c.into(), false).nil();
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    } else {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
