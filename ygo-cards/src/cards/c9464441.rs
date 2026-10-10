//! Adamancipator Risen - Dragite
use super::prelude::*;

pub const ID: u32 = 9464441;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Excavate and add to hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // Negate Spell/Trap or effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 4;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 0, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 5 {
        return;
    }
    duel::confirm_decktop(d, a.tp, 5);
    let hg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    let g = duel::get_decktop_group(d, a.tp, 5);
    let ct = hg.len().min(group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_ROCK), NO_EX));
    if ct > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        let sg = group::select(d, &hg, a.tp, 1, ct as u16, false, NO_EX).nil();
        duel::hint_selection(d, sg.clone(), true);
        if duel::send_to_hand(d, sg, None, REASON_EFFECT, None) != 0 {
            duel::shuffle_hand(d, 1 - a.tp);
        }
        duel::disable_shuffle_check(d, true);
    }
    duel::move_decktop_to_bottom(d, 5, a.tp);
    duel::sort_deckbottom(d, a.tp, a.tp, 5);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && a.ep == 1 - a.tp
        && effect::is_spell_trap_effect(d, re)
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_attribute(d, c, ATTRIBUTE_WATER)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
