//! Magical Plant Mandragola
use super::prelude::*;

pub const ID: u32 = 7802006;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_list: &[COUNTER_SPELL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
    }
}
