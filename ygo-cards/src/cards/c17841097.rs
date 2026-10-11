//! Needle Soldier
use super::prelude::*;

pub const ID: u32 = 17841097;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pierce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    // e1:SetCondition(s.pscon)
    effect::set_condition(d, e1, Some(condition(pscon)));
    // e1:SetOperation(s.psop)
    effect::set_operation(d, e1, Some(operation(psop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.pscon(e,tp,eg,ep,ev,re,r,rp)
fn pscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && a.r == REASON_SYNCHRO
}
// function s.psop(e,tp,eg,ep,ev,re,r,rp)
fn psop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    // local rc=e:GetHandler():GetReasonCard()
    let rc = card::get_reason_card(d, h);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1)
    card::register_effect(d, rc.nil(), e1, false);
}
