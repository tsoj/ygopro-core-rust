//! Darklord Edeh Arae
use super::prelude::*;

pub const ID: u32 = 47664723;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsum success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetOperation(s.gete)
    effect::set_operation(d, e1, Some(operation(gete)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.gete(e,tp,eg,ep,ev,re,r,rp)`
fn gete(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():GetPreviousLocation()~=LOCATION_GRAVE then return end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_previous_location(d, h) != LOCATION_GRAVE {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
