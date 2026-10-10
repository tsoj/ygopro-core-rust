//! Ultimate Dragonic Utopia Ray
use super::prelude::*;

pub const ID: u32 = 75402014;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZW], xyz_number: Some(39), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Equip 1 "ZW -" monster from hand or deck to this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);

    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e2, Some(condition(eqcon)));
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);

    // Negate opponent's cards, up to number of "ZW -" monsters equipped
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DISABLE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId, tc: CardId, tp: u8) -> bool {
    if !(card::is_set_card(d, c, &[SET_ZW as u16]) && !card::is_forbidden(d, c)) {
        return false;
    }
    for te in card::get_card_effect(d, c, ID) {
        let f = effect::get_value(d, te);
        let f = f.as_fn().cloned().nil();
        if f.call(d, &[Value::Card(tc), Value::Card(c), Value::Int(tp as i64)]).truthy() {
            return true;
        }
    }
    false
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, h, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 || card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| filter(d, cc, c, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        let eff = card::get_card_effect(d, tc, ID).first().copied().nil();
        let op = effect::get_operation(d, eff).nil();
        let label_obj = effect::get_label_object(d, eff);
        op.call(d, &[Value::Card(tc), label_obj, Value::Int(tp as i64), Value::Card(c)]);
    }
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(re) = a.re else { return false };
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    tg[0].as_group().is_some_and(|g| group::is_contains(g, h))
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ZW as u16]) && card::is_original_type(d, c, TYPE_MONSTER)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_negatable(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_ONFIELD);
    }
    let h = effect::get_handler(d, a.e).nil();
    let eq = card::get_equip_group(d, h);
    if a.chk == Some(0) {
        return group::is_exists(d, &eq, &cfilter, 1, NO_EX) && duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    let ct = group::filter_count(d, &eq, &cfilter, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);

        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);

        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e3, false);
        }
    }
}
