//! Magical Cylinders
use super::prelude::*;

pub const ID: u32 = 15943341;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[62279055], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Double the damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(dmgcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_operation(d, e2, Some(operation(dmgop)));
    card::register_effect(d, c, e2, false);
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[62279055]) && card::is_s_setable(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(setfilter)),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if g.len() > 0 && duel::s_set(d, a.tp, g.clone(), None, true) > 0 {
        let tc = group::get_first(d, &g).nil();
        if card::is_previous_location(d, tc, LOCATION_DECK) {
            // Can be activated the turn it was Set
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, aux::stringid(ID, 2));
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
fn dmgcon(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != a.tp {
        return false;
    }
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    card::is_code(d, rh, &[62279055]) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn dmgop(d: &mut Duel, a: &EffArgs) {
    // Double the damage of "Magic Cylinder"
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_label_object(d, e1, a.re.map_or(Value::Nil, Value::Effect));
    effect::set_value_fn(d, e1, lua_fn(damval));
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    duel::register_effect(d, e1, a.tp);
}
fn damval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    let lo = effect::get_label_object(d, e).as_effect();
    if (r & REASON_EFFECT) == 0 || a.v(1).as_effect() != lo {
        return val;
    }
    val * 2
}
