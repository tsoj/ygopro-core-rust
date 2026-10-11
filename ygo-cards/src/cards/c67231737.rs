//! Link Bumper
use super::prelude::*;

pub const ID: u32 = 67231737;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, move |d| {
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        aux::add_values_reset(d, reset_s);
    });
}
/// `aux.AddValuesReset(function() s[0]=0 s[1]=0 end)`
fn reset_s(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    if card::get_flag_effect(d, tc, ID) == 0 {
        let key = a.ep as u64;
        let v = s_var(d, ID, key).int();
        set_s_var(d, ID, key, Value::Int(v + 1));
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let at = duel::get_attacker(d).nil();
    if card::is_controler(d, at, 1 - tp) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, c);
    let bt = card::get_battle_target(d, at);
    if !group::is_contains(&lg, at) {
        return false;
    }
    match bt {
        Some(bt) => card::is_controler(d, bt, 1 - tp),
        None => false,
    }
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let at = duel::get_attacker(d);
    if a.chk == Some(0) {
        return s_var(d, ID, tp as u64).int() == 0 || card::get_flag_effect(d, at.nil(), ID) != 0;
    }
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_OATH + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(ftarget)));
    let fid = card::get_field_id(d, at.nil());
    effect::set_label(d, e1, &[fid as i64]);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    true
}
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_link_monster(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_MZONE, 0, c.into()) > 0;
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let gc = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_MZONE, 0, c.into());
    if gc == 0 {
        return;
    }
    let at = duel::get_attacker(d).nil();
    if card::is_relate_to_battle(d, at) && card::is_faceup(d, at) {
        let e0 = effect::create_effect(d, c);
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e0, EFFECT_EXTRA_ATTACK_MONSTER);
        effect::set_value(d, e0, gc as i64);
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
        card::register_effect(d, at, e0, false);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_MUST_ATTACK_MONSTER);
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        effect::set_value_fn(d, e1, effect_card_fn(attg));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE, 1);
        duel::register_effect(d, e1, tp);
    }
}
fn attg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_link_monster(d, c)
}
