//! Penguin Squire
use super::prelude::*;

pub const ID: u32 = 14761450;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PENGUIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_value_fn(d, e1, lua_fn(synlimit));
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand, then you can reduce its Level by 1 or 2.
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_LVCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_CHANGE_POS);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon1)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_MSET);
    effect::set_condition(d, e3, Some(condition(spcon2)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e2);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e4, Some(condition(spcon3)));
    card::register_effect(d, c, e4, false);
    // Change Position
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_count_limit(d, e5, 1, ID, 1, 0);
    effect::set_target(d, e5, Some(cost_target(postg)));
    effect::set_operation(d, e5, Some(operation(posop)));
    card::register_effect(d, c, e5, false);
}
/// `function(e,c)`
fn synlimit(d: &mut Duel, a: Args) -> bool {
    match a.card(1) {
        None => false,
        Some(c) => !card::is_attribute(d, c, ATTRIBUTE_WATER),
    }
}
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_position(d, c, POS_FACEUP as u32) && card::is_facedown(d, c) && card::is_controler(d, c, tp)
}
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter1(d, c, a.tp), 1, NO_EX)
}
fn filter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_facedown(d, c) && card::is_controler(d, c, tp)
}
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter2(d, c, a.tp), 1, NO_EX)
}
fn filter3(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_facedown(d, c) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE)
}
fn spcon3(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter3(d, c, a.tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2) as u64)
    {
        // Reduce level
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
        let lv = duel::announce_number_range(d, a.tp, Some(1), Some(2), &[]).nil().0 as i64;
        let e1 = effect::create_effect(d, c);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e1, -lv);
        card::register_effect(d, c, e1, false);
    }
}
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_defense_pos(d, c) && card::is_can_change_position(d, c)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && posfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&posfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEDOWNDEFENSE as u64);
    let _ = duel::select_target(d, a.tp, Some(&posfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false) > 0
        && card::is_faceup(d, tc)
        && !card::is_set_card(d, tc, &[SET_PENGUIN as u16])
    {
        // Negate its effects
        card::negate_effects(d, tc, c, Some(RESET_EVENT | RESETS_STANDARD), false, None);
    }
}
