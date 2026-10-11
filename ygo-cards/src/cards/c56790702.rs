//! D.D. Assault Carrier
use super::prelude::*;

pub const ID: u32 = 56790702;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Attack cost
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_ATTACK_COST);
    effect::set_cost(d, e1, Some(lua_fn(atkcost)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Special Summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.atkcost(e,c,tp)`
fn atkcost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let coststatus = duel::is_attack_cost_paid(d);
    if coststatus != 2 && card::is_location(d, c, LOCATION_MZONE) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        let cg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), a.tp, LOCATION_GRAVE, 0, NO_EX);
        let tc = group::select_unselect(d, &cg, Some(&CardSet::new()), a.tp, coststatus == 0, coststatus == 0, None, None);
        if let Some(tc) = tc {
            duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None);
            duel::attack_cost_paid(d, None);
        } else {
            duel::attack_cost_paid(d, Some(2));
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_REMOVED, 0) == 3
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
