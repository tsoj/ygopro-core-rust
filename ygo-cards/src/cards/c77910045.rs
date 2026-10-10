//! Fatal Abacus
use super::prelude::*;

pub const ID: u32 = 77910045;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsControler(tp) and c:IsMonster()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp) && card::is_monster(d, c)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=eg:FilterCount(s.filter,nil,tp)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| filter(d, c, tp), NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.Damage(tp,500*ct,REASON_EFFECT)
        duel::damage(d, tp, 500 * ct as i64, REASON_EFFECT, false, None);
    }
    // ct=eg:FilterCount(s.filter,nil,1-tp)
    let ct = group::filter_count(d, &eg, &|d, c| filter(d, c, 1 - tp), NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.Damage(1-tp,500*ct,REASON_EFFECT)
        duel::damage(d, 1 - tp, 500 * ct as i64, REASON_EFFECT, false, None);
    }
}
