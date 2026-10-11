//! True King Agnimazud, the Vanisher
use super::prelude::*;

pub const ID: u32 = 96746083;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 2 monsters and Special Summon itself from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Add 1 non-FIRE Wyrm from the GY to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_EFFECT)
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
fn desfilter2(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::get_sequence(d, c) < 5
}
fn mzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5
}
/// The `ft` and the `g` of `sptg` and `spop`: the cards that can be destroyed for the summon.
fn destroy_targets(d: &mut Duel, tp: u8, c: CardId) -> (i32, CardSet) {
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let g = if ft > -1 {
        let loc = if duel::is_player_affected_by_effect(d, tp, 88581108).is_some() { LOCATION_MZONE } else { 0 };
        duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE | LOCATION_HAND, loc, c.into())
    } else {
        duel::get_matching_group(d, Some(&desfilter2), tp, LOCATION_MZONE, 0, c.into())
    };
    (ft, g)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let (ft, g) = destroy_targets(d, tp, c);
    if a.chk == Some(0) {
        return ft > -2
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && g.len() >= 2
            && group::is_exists(d, &g, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_FIRE), 1, NO_EX)
            && (ft != 0 || group::is_exists(d, &g, &|d, cc| mzfilter(d, cc, tp), 1, NO_EX));
    }
    if g.len() == 2 && group::filter_count(d, &g, &|d, cc| card::is_location(d, cc, LOCATION_HAND), NO_EX) <= 1 {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    } else {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 2, tp, LOCATION_MZONE as i32);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove(d, c, Some(tp), None, None)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let (ft, g) = destroy_targets(d, tp, c);
    if g.len() < 2 || !group::is_exists(d, &g, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_FIRE), 1, NO_EX) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let mut g1 = if ft == 0 {
        group::filter_select(d, &g, tp, &|d, cc| mzfilter(d, cc, tp), 1, 1, false, NO_EX).nil()
    } else {
        group::select(d, &g, tp, 1, 1, false, NO_EX).nil()
    };
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let first = group::get_first(d, &g1).nil();
    if card::is_attribute(d, first, ATTRIBUTE_FIRE) {
        let g2 = group::select(d, &g, tp, 1, 1, false, first.into()).nil();
        group::add_card(&mut g1, &g2);
    } else {
        let g2 = group::filter_select(d, &g, tp, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_FIRE), 1, 1, false, first.into()).nil();
        group::add_card(&mut g1, &g2);
    }
    let rm = group::is_exists(d, &g1, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_FIRE), 2, NO_EX);
    if duel::destroy(d, g1, REASON_EFFECT, None, None) == 2 {
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        let rg = duel::get_matching_group(d, Some(&|d, cc| rmfilter(d, cc, tp)), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
        if rm && !rg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            let tg = group::select(d, &rg, tp, 1, 1, false, NO_EX).nil();
            duel::hint_selection(d, tg.clone(), true);
            duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute_except(d, c, ATTRIBUTE_FIRE, None, 0, None) && card::is_race(d, c, RACE_WYRM) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
