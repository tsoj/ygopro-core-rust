//! Rose Paladin
use super::prelude::*;

pub const ID: u32 = 96385345;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bc = card::get_battle_target(d, c).nil();
    card::is_monster(d, bc) && card::is_location(d, bc, LOCATION_GRAVE)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_PLANT)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_above(d, c, 7) && card::is_race(d, c, RACE_PLANT) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
