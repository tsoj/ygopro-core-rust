//! Witch's Strike
use super::prelude::*;

pub const ID: u32 = 36458063;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON_NEGATED);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_NEGATED);
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    card::register_effect(d, c, e3, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, move |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_CHAIN_NEGATED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_DISABLE_REASON, CHAININFO_DISABLE_PLAYER]);
    let de = info[0].clone();
    let dp = info[1].int() as u8;
    if !de.is_nil() {
        let c = effect::get_handler(d, a.e).nil();
        duel::raise_event(d, CardSet::from_card(c), EVENT_CUSTOM + ID, Some(a.e), 0, dp, 0, 0);
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD | LOCATION_HAND);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
