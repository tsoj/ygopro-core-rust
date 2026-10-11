//! Outburst Dragon
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 87263576;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon and destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Link Summon during opponent's Main Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(turn_cond)));
    effect::set_target(d, e2, Some(cost_target(lktg)));
    effect::set_operation(d, e2, Some(operation(lkop)));
    card::register_effect(d, c, e2, false);
}
fn turn_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !g.is_empty()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None);
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, 0, c.into());
        if !g.is_empty() {
            duel::break_effect(d);
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
        // Cannot Special Summon non-DARK monsters
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_absolute_range(d, e1, tp, 1, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_target(d, e1, Some(effect_card_fn(not_dark_target)));
        card::register_effect(d, c, e1, false);
    }
}
fn not_dark_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_attribute(d, c, ATTRIBUTE_DARK)
}
fn lkfilter(d: &mut Duel, c: CardId, mc: CardId) -> bool {
    card::is_race(d, c, RACE_DRAGON) && card::is_link_summonable(d, c, Some(CardSet::from_card(mc)), None, 0, 0)
}
fn lktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| lkfilter(d, c, h)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn lkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - tp) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, x| lkfilter(d, x, c)), tp, LOCATION_EXTRA, 0, NO_EX);
    if !g.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &sel) {
            duel::link_summon(d, tp, sc, None, Some(CardSet::from_card(c)), 0, 0);
        }
    }
}
