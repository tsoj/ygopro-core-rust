//! All-Eyes Phantom Dragon
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 70335319;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    pendulum::add_procedure(d, c, None, None);
    // Chain attack
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cacon)));
    effect::set_cost(d, e1, Some(cost_target(cacost)));
    effect::set_target(d, e1, Some(cost_target(catg)));
    effect::set_operation(d, e1, Some(operation(caop)));
    card::register_effect(d, c, e1, false);
    // Special Summon condition
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e2, false);
    // Special summon procedure
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_range(d, e3, LOCATION_HAND);
    effect::set_count_limit(d, e3, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e3, Some(lua_fn(spcon)));
    effect::set_operation(d, e3, Some(lua_fn(spop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_range(d, e4, LOCATION_EXTRA);
    card::register_effect(d, c, e4, false);
    // ATK change
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 2));
    effect::set_category(d, e5, CATEGORY_ATKCHANGE);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_code(d, e5, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(atkcon)));
    effect::set_operation(d, e5, Some(operation(atkop)));
    card::register_effect(d, c, e5, false);
    // Negate
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 3));
    effect::set_category(d, e6, CATEGORY_NEGATE);
    effect::set_type(d, e6, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e6, EVENT_CHAINING);
    effect::set_property(d, e6, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_range(d, e6, LOCATION_MZONE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_condition(d, e6, Some(condition(discon)));
    effect::set_cost(d, e6, Some(cost_target(discost)));
    effect::set_target(d, e6, Some(cost_target(distg)));
    effect::set_operation(d, e6, Some(operation(disop)));
    card::register_effect(d, c, e6, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
    // aux.GlobalCheck(s, function() s[0]=0 s[1]=0 ... end)
    aux::global_check(d, ID, move |d| {
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        aux::add_values_reset(d, values_reset);
    });
}
fn values_reset(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let tc = group::get_first(d, &a.eg()).nil();
    if card::get_flag_effect(d, tc, ID) == 0 {
        let key = a.ep as u64;
        let cur = s_var(d, ID, key).int();
        set_s_var(d, ID, key, Value::Int(cur + 1));
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
fn pfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_PENDULUM) && card::is_race(d, c, RACE_DRAGON)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_sequence(d, c) < 5
}
fn spcon(d: &mut Duel, a: Args) -> Value {
    let Some(c) = a.card(1) else {
        return Value::Bool(true);
    };
    let tp = card::get_controler(d, c);
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    let rg = duel::get_release_group(d, tp, false, false, None);
    let res = (!g.is_empty() || !rg.is_empty())
        && group::filter_count(d, &g, &|d, x| card::is_releasable(d, x, None, None), NO_EX) == g.len()
        && g.len() > 1
        && group::is_exists(d, &g, &pfilter, 1, NO_EX)
        && group::filter_count(d, &g, &cfilter, NO_EX) as i32 + duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    Value::Bool(res)
}
fn spop(d: &mut Duel, a: Args) -> Value {
    let tp = a.p(1);
    let g = duel::get_release_group(d, tp, false, false, None);
    duel::release(d, g, REASON_COST, None);
    Value::Nil
}
fn cacon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    match duel::get_attacker(d) {
        Some(at) => {
            card::is_faceup(d, at)
                && pfilter(d, at)
                && card::is_controler(d, at, tp)
                && duel::get_attack_target(d).is_some()
                && card::can_chain_attack(d, at, Some(2), false)
        }
        None => false,
    }
}
fn cacfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::get_flag_effect(d, c, ID) != 0
}
fn cacost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let at = duel::get_attacker(d);
    if a.chk == Some(0) {
        let ex = match at {
            Some(x) => x.into(),
            None => NO_EX,
        };
        return duel::get_matching_group_count(d, Some(&cacfilter), tp, 0xff, 0xff, ex) == 0;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_OATH + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(ftarget)));
    let fid = card::get_field_id(d, at.nil());
    effect::set_label(d, e1, &[fid as i64]);
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    true
}
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
fn catg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return match duel::get_attacker(d) {
            Some(at) => card::can_chain_attack(d, at, Some(2), false),
            None => false,
        };
    }
    true
}
fn caop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let at = duel::get_attacker(d);
    if let Some(at) = at {
        if card::is_relate_to_battle(d, at) && card::is_controler(d, at, tp) {
            duel::chain_attack(d, None);
        }
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_battle_target(d, c).is_some()
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, c) as i64;
        effect::set_value(d, e1, atk * 2);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    a.ep != a.tp && effect::is_spell_trap_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&disfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&disfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_activation(d, a.ev as u8);
}
