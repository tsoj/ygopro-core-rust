//! Retort
use super::prelude::*;

pub const ID: u32 = 22747316;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    a.rp != a.tp
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
        && {
            let h = effect::get_handler(d, re).nil();
            let codes = card::get_code(d, h);
            duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &codes)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let re = a.re.nil();
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let h = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, h, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn filter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    let code = card::get_code(d, h)[0];
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, h, re) {
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
    let nv = aux::necro_valley_filter(|d, c| filter(d, c, code));
    let g = duel::get_matching_group(d, Some(&nv), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
