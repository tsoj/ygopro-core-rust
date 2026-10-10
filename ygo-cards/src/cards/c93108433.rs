//! Monster Recovery
use super::prelude::*;

pub const ID: u32 = 93108433;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn mfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_able_to_deck(d, c) && tp == card::get_owner(d, c)
}
fn hfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    tp != card::get_owner(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && mfilter(d, chkc, a.tp);
    }
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_player_can_draw(d, a.tp, 0)
            && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0
            && duel::is_existing_target(d, Some(&|d, c| mfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, h.into())
            && !duel::is_existing_matching_card(d, Some(&|d, c| hfilter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let mut g1 = duel::select_target(d, a.tp, Some(&|d, c| mfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let g2 = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g1.clone()), g1.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, g2.len() as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    let mut g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    let ct = g.len();
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && ct > 0 {
            let one = CardSet::from(tc);
            group::add_card(&mut g, &one);
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            duel::shuffle_deck(d, a.tp);
            if ct > 0 {
                duel::break_effect(d);
                duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
            }
        }
    }
}
