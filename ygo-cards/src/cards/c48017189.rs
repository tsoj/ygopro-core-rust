//! EM:P Meowmine
use super::prelude::*;

pub const ID: u32 = 48017189;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // Return an opponent's card to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_GRAVE | LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(rthcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    effect::set_operation(d, e2, Some(operation(rthop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_REMOVE);
    card::register_effect(d, c, e3, false);
}
/// `function(e,c)`: the core checks the effect alone with no card
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&card::is_link_monster), 0, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_link_monster(d, c) && card::is_link(d, c, &[2]) && card::is_faceup(d, c) && card::is_previous_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_MZONE)
}
fn rthcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// Activation legality
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_hand(d, chkc, None);
    }
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && ((card::is_location(d, c, LOCATION_GRAVE) && !group::is_contains(&eg, c)) || card::is_location(d, c, LOCATION_HAND));
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, a.tp, 0);
    true
}
fn rthop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
