//! Shadow-Imprisoning Mirror
use super::prelude::*;

pub const ID: u32 = 99735427;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAIN_ACTIVATING)
    effect::set_code(d, e2, EVENT_CHAIN_ACTIVATING);
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
    let loc = info[0].int() as u32;
    // if re:IsMonsterEffect() and (loc==LOCATION_MZONE or loc==LOCATION_GRAVE)
    //     and re:GetHandler():IsAttribute(ATTRIBUTE_DARK) then
    let re = a.re.nil();
    if effect::is_monster_effect(d, re) && (loc == LOCATION_MZONE || loc == LOCATION_GRAVE) {
        let rh = effect::get_handler(d, re).nil();
        if card::is_attribute(d, rh, ATTRIBUTE_DARK) {
            // Duel.NegateEffect(ev)
            duel::negate_effect(d, a.ev as u8);
        }
    }
}
