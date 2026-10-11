//! Gwenhwyfar, Queen of Noble Arms
use super::prelude::*;

pub const ID: u32 = 19748583;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NOBLE_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // atkup
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 300);
    card::register_effect(d, c, e2, false);
    // destroy sub
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_target(d, e3, Some(cost_target(reptg)));
    effect::set_operation(d, e3, Some(operation(repop)));
    card::register_effect(d, c, e3, false);
    // destroy
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_code(d, e4, EVENT_BATTLE_START);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    if card::is_location(d, c, LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    }
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    let mut bad = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0;
    if !bad {
        let tc = tc.nil();
        bad = card::get_controler(d, tc) != a.tp || card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e);
    }
    if bad {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    let tc = tc.nil();
    duel::equip(d, a.tp, c, tc, true, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])));
    card::register_effect(d, c, e1, false);
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let reason_effect = (a.r & REASON_EFFECT) != 0;
        if !reason_effect {
            return false;
        }
        let et = card::get_equip_target(d, c).nil();
        return card::is_attribute(d, et, ATTRIBUTE_LIGHT)
            && !card::is_reason(d, c, REASON_REPLACE)
            && card::is_destructable(d, c, Some(a.e))
            && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED);
    }
    let owner = effect::get_owner_player(d, a.e);
    duel::select_effect_yes_no(d, owner, c, Some(96))
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT | REASON_REPLACE, None, None);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    match card::get_equip_target(d, c) {
        Some(tg) => {
            let dark = card::is_attribute(d, tg, ATTRIBUTE_DARK);
            dark && (duel::get_attacker(d) == Some(tg) || duel::get_attack_target(d) == Some(tg))
        }
        None => false,
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, c).nil();
    let tc = card::get_battle_target(d, et);
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::is_controler(d, tc, 1 - a.tp),
            None => false,
        };
    }
    // Group.FromCards(tc,c)
    let mut g = CardSet::new();
    if let Some(tc) = tc {
        group::add_card(&mut g, &CardSet::from_card(tc));
    }
    group::add_card(&mut g, &CardSet::from_card(c));
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let et = card::get_equip_target(d, c).nil();
    let tc = card::get_battle_target(d, et).nil();
    if card::is_relate_to_battle(d, tc) && duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
        duel::break_effect(d);
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
