//! Spatial Collapse
use super::prelude::*;

pub const ID: u32 = 20644748;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    card::register_effect(d, c, e1, false);

    // adjust
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EVENT_ADJUST);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(adjustop)));
    card::register_effect(d, c, e2, false);

    //
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EFFECT_MAX_MZONE);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 1, 1);
    effect::set_value_fn(d, e3, lua_fn(mvalue));
    card::register_effect(d, c, e3, false);

    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EFFECT_MAX_SZONE);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e4, 1, 1);
    effect::set_value_fn(d, e4, lua_fn(svalue));
    card::register_effect(d, c, e4, false);

    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_code(d, e5, EFFECT_CANNOT_ACTIVATE);
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e5, 1, 1);
    effect::set_value_fn(d, e5, lua_fn(aclimit));
    card::register_effect(d, c, e5, false);

    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    effect::set_range(d, e6, LOCATION_SZONE);
    effect::set_code(d, e6, EFFECT_CANNOT_SSET);
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e6, 1, 1);
    effect::set_target(d, e6, Some(lua_fn(setlimit)));
    card::register_effect(d, c, e6, false);
}

fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) <= 5 && duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) <= 5
}

fn adjustop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let phase = duel::get_current_phase(d);
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    let c1 = duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0);
    let c2 = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    if c1 > 5 || c2 > 5 {
        let mut g = CardSet::new();
        if c1 > 5 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            let n = (c1 - 5) as u16;
            let g1 = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, 0, n, n, false, NO_EX).nil();
            group::add_card(&mut g, &g1);
        }
        if c2 > 5 {
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
            let n = (c2 - 5) as u16;
            let g2 = duel::select_matching_card(d, 1 - tp, None, 1 - tp, LOCATION_ONFIELD, 0, n, n, false, NO_EX).nil();
            group::add_card(&mut g, &g2);
        }
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        duel::readjust(d);
    }
}

/// `s.mvalue(e,fp,rp,r)`
fn mvalue(d: &mut Duel, a: Args) -> i64 {
    let fp = a.int(1) as u8;
    5 - duel::get_field_group_count(d, fp, LOCATION_SZONE, 0) as i64
}

/// `s.svalue(e,fp,rp,r)`
fn svalue(d: &mut Duel, a: Args) -> i64 {
    let fp = a.int(1) as u8;
    5 - duel::get_field_group_count(d, fp, LOCATION_MZONE | LOCATION_FZONE, 0) as i64
}

/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let tp = a.int(2) as u8;
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    if effect::is_active_type(d, re, TYPE_FIELD) {
        return duel::get_field_card(d, tp, LOCATION_FZONE, 0).is_none() && duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) > 4;
    } else if effect::is_active_type(d, re, TYPE_PENDULUM) {
        return duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) > 4;
    }
    false
}

/// `s.setlimit(e,c,tp)`
fn setlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let tp = a.int(2) as u8;
    card::is_field_spell(d, c) && duel::get_field_card(d, tp, LOCATION_FZONE, 0).is_none() && duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) > 4
}
