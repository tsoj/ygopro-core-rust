//! Constellar Belt
use super::prelude::*;

pub const ID: u32 = 86825483;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // inactivatable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_INACTIVATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.efilter(e,ct)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local te=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT)
    let te = duel::get_chain_info(d, a.int(1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te);
    // return te:IsMonsterEffect() and tc:IsAttribute(ATTRIBUTE_LIGHT)
    effect::is_monster_effect(d, te) && card::is_attribute(d, tc.nil(), ATTRIBUTE_LIGHT)
}
