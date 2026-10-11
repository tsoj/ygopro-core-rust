//! Eternal Dread
use super::prelude::*;

pub const ID: u32 = 35787450;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[0x1b],
    listed_names: &[75041269],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.addcon)
    effect::set_condition(d, e1, Some(condition(addcon)));
    // e1:SetOperation(s.addc)
    effect::set_operation(d, e1, Some(operation(addc)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.check(tp)`
fn check(d: &mut Duel, tp: u8) -> bool {
    // local tc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let tc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
    // return tc and tc:IsFaceup() and tc:IsCode(75041269)
    match tc {
        Some(tc) => card::is_faceup(d, tc) && card::is_code(d, tc, &[75041269]),
        None => false,
    }
}
/// `s.addcon(e,tp,eg,ep,ev,re,r,rp)`
fn addcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return s.check(0) or s.check(1)
    check(d, 0) || check(d, 1)
}
/// `s.addc(e,tp,eg,ep,ev,re,r,rp)`
fn addc(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let tc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
    // if tc and tc:IsFaceup() and tc:IsCode(75041269) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_code(d, tc, &[75041269]) {
            // tc:AddCounter(0x1b,2)
            card::add_counter(d, tc, 0x1b, 2, false);
        }
    }
    // tc=Duel.GetFieldCard(1-tp,LOCATION_FZONE,0)
    let tc = duel::get_field_card(d, 1 - tp, LOCATION_FZONE, 0);
    // if tc and tc:IsFaceup() and tc:IsCode(75041269) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_code(d, tc, &[75041269]) {
            // tc:AddCounter(0x1b,2)
            card::add_counter(d, tc, 0x1b, 2, false);
        }
    }
}
