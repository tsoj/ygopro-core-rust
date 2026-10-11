//! Dragon Mastery
use super::prelude::*;

pub const ID: u32 = 99659159;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
    // equip
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_EQUIP);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
fn equipf(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_monster_card(d, c)
}
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    let eg = card::get_equip_group(d, c);
    group::is_exists(d, &eg, &equipf, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_race(d, c, RACE_DRAGON) && !card::is_forbidden(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.card(1);
    effect::get_label_object(d, e).as_card() == c
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let sg = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if sg.is_empty() {
        return;
    }
    let sc = group::get_first(d, &sg).nil();
    duel::equip(d, tp, sc, tc, true, false);
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value_fn(d, e1, lua_fn(eqlimit));
    effect::set_label_object(d, e1, Value::Card(tc));
    card::register_effect(d, sc, e1, false);
}
