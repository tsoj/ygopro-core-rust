//! Sky Striker Mecha - Widow Anchor
use super::prelude::*;

pub const ID: u32 = 98338152;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the effects of a face-up monster on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_location(d, c, LOCATION_MMZONE)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && !card::is_disabled(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    if duel::get_matching_group_count(d, Some(&|d, c| card::is_spell(d, c)), tp, LOCATION_GRAVE, 0, NO_EX) >= 3 {
        effect::set_category(d, a.e, CATEGORY_DISABLE | CATEGORY_CONTROL);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_disabled(d, tc) {
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
        duel::adjust_instantly(d, Some(tc));
        if duel::get_matching_group_count(d, Some(&|d, c| card::is_spell(d, c)), tp, LOCATION_GRAVE, 0, NO_EX) >= 3
            && card::is_controler_can_be_changed(d, tc, false, None)
            && card::is_controler(d, tc, 1 - tp)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            duel::break_effect(d);
            duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None);
        }
    }
}
