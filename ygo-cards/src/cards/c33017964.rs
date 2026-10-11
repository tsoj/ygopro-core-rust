//! Illusion Gate
use super::prelude::*;

pub const ID: u32 = 33017964;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy as many monsters your opponent controls as possible, then you can Special Summon 1 monster from your opponent's GY to your field, ignoring its Summoning conditions
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Fraction(0.5))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, tp, ID, None) && !g.is_empty();
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, LOCATION_GRAVE as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    duel::register_flag_effect(d, tp, ID, 0, 0, 1, 0);
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty()
        && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, 0, LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = duel::select_target(d, tp, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::break_effect(d);
            duel::special_summon(d, sg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
