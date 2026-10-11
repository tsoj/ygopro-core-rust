//! H-C War Hammer
//! Heroic Challenger - War Hammer
use super::prelude::*;

pub const ID: u32 = 26885836;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_condition(d, e1, Some(condition(eqcon)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    aux::add_er_equip_limit(
        d,
        c,
        Some(lua_fn(|d, a| eqconc(d, a.e(0)))),
        lua_fn(|d, a| card::is_controler(d, a.c(0), 1 - a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
}
fn eqconc(d: &mut Duel, e: EffectId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let g = card::get_equip_group(d, h);
    let f = group::filter(d, &g, &eqfilter, NO_EX);
    f.is_empty()
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    aux::bdogcon().call(d, &[Value::Effect(a.e), Value::Int(a.tp as i64), a.eg.clone().map_or(Value::Nil, Value::Group), Value::Int(a.ep as i64), Value::Int(a.ev as i64), Value::effect(a.re), Value::Int(a.r as i64), Value::Int(a.rp as i64)]).truthy()
        && !card::is_forbidden(d, tc.nil())
        && eqconc(d, a.e)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, tc.map(|x| x.into()), 1, 0, 0);
    true
}
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    let mut atk = card::get_base_attack(d, tc);
    if atk < 0 {
        atk = 0;
    }
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None) {
        return;
    }
    if atk > 0 {
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e2, atk as i64);
        card::register_effect(d, tc, e2, false);
    }
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                equipop(d, c, a.e, a.tp, tc);
            }
        }
    }
}
