//! Gorgonic Umbral Horror
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER, SET_UMBRAL_HORROR], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 76548531;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You cannot Special Summon from the Extra Deck, except "Number" Xyz Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !(card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_xyz_monster(d, c)))));
    card::register_effect(d, c, e1, false);
    aux::add_continuous_lizard_check(
        d,
        c,
        LOCATION_MZONE,
        Some(effect_card_fn(|d, _e, c| !(card::is_original_type(d, c, TYPE_XYZ) && card::is_original_set_card(d, c, &[SET_NUMBER as u16])))),
        None,
        None,
    );
    // If you control no monsters, or all monsters you control are "Umbral Horror" monsters: You can Special Summon this card from your hand, and if you do, add 1 non-Rock "Umbral Horror" monster from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON | CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Can be treated as 2 materials for the Xyz Summon of a "Number" Xyz Monster that requires 3 or more materials
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_DOUBLE_XYZ_MATERIAL);
    effect::set_value(d, e3, 1);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_operation(d, e3, Some(lua_fn(e3_op)));
    card::register_effect(d, c, e3, false);
}
fn e3_op(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let minct = crate::class::get(d, c, crate::class::MIN_XYZ_CT);
    minct.truthy() && minct.int() >= 3 && card::is_set_card(d, c, &[SET_NUMBER as u16])
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    g.is_empty() || g.len() == group::filter_count(d, &g, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16]), NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16]) && card::is_monster(d, c) && !card::is_race(d, c, RACE_ROCK) && card::is_able_to_hand(d, c, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let sc = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !sc.is_empty() && duel::send_to_hand(d, sc.clone(), Some(a.tp), REASON_EFFECT, None) > 0 {
            duel::confirm_cards(d, 1 - a.tp, sc);
        }
    }
}
