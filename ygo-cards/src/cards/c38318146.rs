//! Tyrant's Tantrum
//! 暴君の暴力
use super::prelude::*;

pub const ID: u32 = 38318146;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Players must send 1 Spell card from the Deck to activate Spell cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_ACTIVATE_COST)
    effect::set_code(d, e2, EFFECT_ACTIVATE_COST);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCost(s.actcost)
    effect::set_cost(d, e2, Some(lua_fn(actcost)));
    // e2:SetTarget(s.actcosttarget)
    effect::set_target(d, e2, Some(lua_fn(actcosttarget)));
    // e2:SetOperation(s.actcostop)
    effect::set_operation(d, e2, Some(operation(actcostop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // Accumulation
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(id)
    effect::set_code(d, e3, ID);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroup(tp,nil,2,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group(d, a.tp, None, 2, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local rg=Duel.SelectReleaseGroup(tp,nil,2,2,nil)
    let rg = duel::select_release_group(d, a.tp, None, 2, 2, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.actvcostfilter(c)`
fn actvcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToGraveAsCost()
    card::is_spell(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.actcost(e,te,tp)`
fn actcost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    // local ct=#{Duel.GetPlayerEffect(tp,id)}
    let ct = duel::get_player_effect(d, tp, ID).len() as u32;
    // return Duel.IsExistingMatchingCard(s.actvcostfilter,tp,LOCATION_DECK,0,ct,nil)
    duel::is_existing_matching_card(d, Some(&actvcostfilter), tp, LOCATION_DECK, 0, ct, NO_EX)
}
/// `s.actcosttarget(e,te,tp)`
fn actcosttarget(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    // return te:IsSpellEffect() and te:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_spell_effect(d, te) && effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE)
}
/// `s.actcostop(e,tp,eg,ep,ev,re,r,rp)`
fn actcostop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=#{Duel.GetPlayerEffect(tp,id)}
    let ct = duel::get_player_effect(d, tp, ID).len() as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.actvcostfilter,tp,LOCATION_DECK,0,ct,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&actvcostfilter), tp, LOCATION_DECK, 0, ct, ct, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
