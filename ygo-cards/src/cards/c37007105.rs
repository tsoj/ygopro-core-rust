//! Cybersal Cyclone
use super::prelude::*;

pub const ID: u32 = 37007105;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 targeted oppo's Link by banishing one with the same rating from your GY, if banished a Cyberse, destroy 1 faceup card in oppo's Spell & Trap zone
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !(card::is_faceup(d, c) && card::is_link_monster(d, c)) {
        return false;
    }
    let link = card::get_link(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| rmfilter(d, cc, link)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn rmfilter(d: &mut Duel, c: CardId, link: u32) -> bool {
    card::is_monster(d, c) && card::is_link(d, c, &[link]) && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    let n = g.as_ref().map_or(0, |x| x.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, n, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let link = card::get_link(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| rmfilter(d, c, link))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if duel::remove(d, g.clone(), 0, REASON_EFFECT, None, None) != 0 {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
        let first = group::get_first(d, &g).nil();
        if card::is_original_race(d, first, RACE_CYBERSE) {
            let sg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_STZONE, NO_EX);
            if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
                duel::break_effect(d);
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                let dg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
                duel::hint_selection(d, dg.clone(), true);
                duel::destroy(d, dg, REASON_EFFECT, None, None);
            }
        }
    }
}
