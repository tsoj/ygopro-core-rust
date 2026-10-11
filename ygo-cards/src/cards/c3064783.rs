//! Shining Umbral Horror
use super::prelude::*;

pub const ID: u32 = 3064783;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UMBRAL_HORROR, SET_MASQUERADE, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you Normal or Special Summon an "Umbral Horror" monster(s) and/or a "Masquerade" Xyz Monster(s): You can Special Summon this card from your hand, and if you do, draw 1 card, then if you drew an "Umbral Horror" monster, you can Special Summon it
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e1a, LOCATION_HAND);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_condition(d, e1a, Some(condition(spcon)));
    effect::set_target(d, e1a, Some(cost_target(sptg)));
    effect::set_operation(d, e1a, Some(operation(spop)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
    // Can be treated as 2 materials for the Xyz Summon of a "Number" Xyz Monster that requires 3 or more materials
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DOUBLE_XYZ_MATERIAL);
    effect::set_value(d, e2, 1);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_operation(d, e2, Some(lua_fn(|d, a| {
        // `c` is the second argument of the call (the Lua parameter named c)
        let c = a.c(1);
        let m = crate::class::get(d, c, crate::class::MIN_XYZ_CT);
        m.truthy() && m.int() >= 3 && card::is_set_card(d, c, &[SET_NUMBER as u16])
    })));
    card::register_effect(d, c, e2, false);
}
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    (card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16]) || (card::is_set_card(d, c, &[SET_MASQUERADE as u16]) && card::is_xyz_monster(d, c)))
        && card::is_summon_player(d, c, tp)
        && card::is_faceup(d, c)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, a.tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::draw(d, a.tp, 1, REASON_EFFECT) > 0
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        let operated = duel::get_operated_group(d);
        let drawn_card = group::get_first(d, &operated).nil();
        if card::is_set_card(d, drawn_card, &[SET_UMBRAL_HORROR as u16]) && card::is_can_be_special_summoned(d, drawn_card, a.e, 0, a.tp, false, false, None, None, None) {
            if !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
                duel::shuffle_hand(d, a.tp);
                return;
            }
            duel::break_effect(d);
            duel::special_summon(d, drawn_card, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
