//! Zubaba General
use super::prelude::*;

pub const ID: u32 = 31563350;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_fn), e1, None, None, None);
}
fn eqval(d: &mut Duel, a: Args) -> bool {
    let ec = a.c(0);
    let tp = a.p(2);
    card::is_controler(d, ec, tp) && card::is_race(d, ec, RACE_WARRIOR)
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_race(d, c, RACE_WARRIOR) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
        return;
    }
    let atk = card::get_text_attack(d, tc);
    if atk > 0 {
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
        effect::set_value(d, e2, atk as i64);
        card::register_effect(d, tc, e2, false);
    }
}
fn equipop_fn(d: &mut Duel, a: Args) {
    let c = a.c(0);
    let e = a.e(1);
    let tp = a.p(2);
    let tc = a.c(3);
    equipop(d, c, e, tp, tc);
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 || card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        equipop(d, c, a.e, a.tp, tc);
    }
}
