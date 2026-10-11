//! ZW - Leo Arms
use super::prelude::*;

pub const ID: u32 = 60992364;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZW, SET_UTOPIA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Xyz summon procedure
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // Cannot attack directly
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    card::register_effect(d, c, e1, false);
    // Add 1 "ZW -" monster from deck
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
    // Equip this card on the field to 1 "Utopia" monster
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e3, CATEGORY_EQUIP);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    effect::set_operation(d, e3, Some(operation(eqop)));
    card::register_effect(d, c, e3, false);
    aux::add_zw_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a: Args| {
            let (tc, tp) = (a.c(0), a.p(2));
            filter(d, tc) && card::is_controler(d, tc, tp)
        }),
        lua_fn(|d, a: Args| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e3,
        None,
        None,
        None,
    );
    // Send this equip card to GY, the monster it was equipped to can make a second attack on monsters
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_condition(d, e4, Some(condition(atcon)));
    effect::set_cost(d, e4, Some(cost_target(atcost)));
    effect::set_target(d, e4, Some(cost_target(attg)));
    effect::set_operation(d, e4, Some(operation(atop)));
    card::register_effect(d, c, e4, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ZW as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0 {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UTOPIA as u16])
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) || card::get_controler(d, tc) != a.tp {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    equipop(d, c, a.e, a.tp, tc);
}
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    if !aux::equip_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 3000);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp) && duel::is_battle_phase(d, None) && duel::get_current_chain(d, false) == 0
}
fn atcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c);
    }
    let eqt = card::get_equip_target(d, c);
    effect::set_label_object(d, a.e, eqt.map_or(Value::Nil, Value::Card));
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let eqc = card::get_equip_target(d, c);
    if a.chk == Some(0) {
        return match eqc {
            Some(eqc) => card::get_attacked_count(d, eqc) == 1 && card::get_effect_count(d, eqc, EFFECT_EXTRA_ATTACK) == 0,
            None => false,
        };
    }
    let target = effect::get_label_object(d, a.e).as_card().nil();
    duel::set_target_card(d, target);
    true
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // Can make a second attack on monsters
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3202);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_CANNOT_DIRECT_ATTACK);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
}
