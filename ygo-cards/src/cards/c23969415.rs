//! Beta Evolution Pill - Ultranscendance
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 23969415;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 level 5 or higher Dinosaur monster from the Deck or Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[-100]);
    true
}
/// `s.spcheck(sg,tp,exg,e)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, e: EffectId) -> bool {
    if !group::is_exists(d, sg, &|d, c| card::is_race(d, c, RACE_DINOSAUR), 1, NO_EX) {
        return false;
    }
    let atk = group::get_sum(d, sg, &|d, c| card::get_attack(d, c) as i64);
    duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, Some(sg), atk)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,sg,atk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: Option<&CardSet>, atk: i64) -> bool {
    if !(card::is_attack_above(d, c, atk as i32) && card::is_race(d, c, RACE_DINOSAUR) && card::is_level_above(d, c, 5)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    let ex = match sg {
        Some(g) => g.into(),
        None => NO_EX,
    };
    if card::is_location(d, c, LOCATION_DECK) {
        duel::get_m_zone_count(d, tp, ex, None, None, None).0 > 0
    } else {
        duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != -100 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        let e = a.e;
        let tp = a.tp;
        return duel::check_release_group_cost(d, tp, None, 2, 2, true, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet| (spcheck(d, sg, tp, e), false)), NO_EX);
    }
    let e = a.e;
    let tp = a.tp;
    let sg = duel::select_release_group_cost(d, tp, None, 2, 2, true, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet| (spcheck(d, sg, tp, e), false)), NO_EX);
    let sum = group::get_sum(d, &sg, &|d, c| card::get_attack(d, c) as i64);
    effect::set_label(d, e, &[sum]);
    duel::release(d, sg, REASON_COST, None);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let label = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None, label)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if group::get_count(&g) > 0 {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // Cannot Special Summon from the Extra Deck, except Dragon, Dinosaur, Sea Serpent, and Wyrm monsters
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard check
    let h = effect::get_handler(d, a.e).nil();
    aux::add_temp_lizard_check(d, h, a.tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_race(d, c, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM)
}
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_race(d, c, RACE_DINOSAUR | RACE_DRAGON | RACE_SEASERPENT | RACE_WYRM)
}
