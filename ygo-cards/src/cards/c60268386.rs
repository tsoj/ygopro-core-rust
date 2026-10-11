//! Heraldic Beast Gryphon
use super::prelude::*;

pub const ID: u32 = 60268386;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERALDIC_BEAST], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Can be treated as 2 Materials for the Xyz Summon of a "Number" Xyz monter that requires 3 or more materials
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DOUBLE_XYZ_MATERIAL);
    effect::set_value(d, e2, 1);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_operation(d, e2, Some(lua_fn(|d, a| {
        // `c` is the second argument of the operation call (the Lua parameter named c)
        let c = a.c(1);
        let m = crate::class::get(d, c, crate::class::MIN_XYZ_CT);
        m.truthy() && m.int() >= 3 && card::is_set_card(d, c, &[SET_NUMBER as u16])
    })));
    card::register_effect(d, c, e2, false);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_HERALDIC_BEAST as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c) && !card::is_code(d, c, &[ID])
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Cannot Special Summon from the Extra Deck for the rest of this turn, except by Xyz Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(|d, a: Args| {
        let c = a.c(1);
        let sumtype = a.int(3) as u64;
        card::is_location(d, c, LOCATION_EXTRA) && (sumtype & SUMMON_TYPE_XYZ as u64) != SUMMON_TYPE_XYZ as u64
    })));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // Can only use monsters with "Heraldic Beast" and/or "Number" in their original names as material for an Xyz Summon this turn
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    effect::set_target_range(d, e2, LOCATION_ALL, LOCATION_ALL);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !card::is_original_set_card(d, c, &[SET_HERALDIC_BEAST as u16, SET_NUMBER as u16]))));
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| card::is_controler(d, c, effect::get_handler_player(d, e))));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
}
