//! Dragon Master Lords
use super::prelude::*;

pub const ID: u32 = 58931850;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 12 monsters
    xyz::add_procedure(d, c, None, Some(12), 2, None, None, None, None, false, None);
    // While you control this Xyz Summoned card, your opponent cannot target it with card effects, also it cannot be destroyed by your opponent's card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    effect::set_value_fn(d, e1, aux::tgoval());
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e2, aux::indoval());
    card::register_effect(d, c, e2, false);
    // If this cards attacks a Defense Position monster while you have 25 or more cards in your GY, inflict double piercing battle damage to your opponent
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_PIERCE);
    effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
    effect::set_value(d, e3, DOUBLE_DAMAGE as i64);
    card::register_effect(d, c, e3, false);
    // Special Summon 1 monster from your Deck, Extra Deck, or GY
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(condition(e4_cond)));
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
}
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_xyz_summoned(d, c)
}
fn e3_cond(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, p, LOCATION_GRAVE, 0) >= 25
}
fn e4_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_GRAVE) >= 25
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    if card::is_location(d, c, LOCATION_EXTRA) {
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
