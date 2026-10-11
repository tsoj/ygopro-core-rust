//! Dinowrestler Martial Ankylo
use super::prelude::*;

pub const ID: u32 = 35770983;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOWRESTLER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk change
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // to grave
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_operation(d, e2, Some(operation(regop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let mut tc = match duel::get_attacker(d) {
        Some(tc) => tc,
        None => return false,
    };
    if card::is_controler(d, tc, 1 - a.tp) {
        tc = match duel::get_attack_target(d) {
            Some(t) => t,
            None => {
                effect::set_label_object(d, a.e, Value::Nil);
                return false;
            }
        };
    }
    effect::set_label_object(d, a.e, Value::Card(tc));
    let h = effect::get_handler(d, a.e).nil();
    tc != h
        && card::is_set_card(d, tc, &[SET_DINOWRESTLER as u16])
        && card::is_relate_to_battle(d, tc)
        && duel::get_attack_target(d).is_some()
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let a_o = duel::get_attacker(d);
    let b_o = card::get_battle_target(d, a_o.nil());
    let (a_o, b_o) = if card::is_controler(d, a_o.nil(), 1 - a.tp) { (b_o, a_o) } else { (a_o, b_o) };
    let at = a_o.nil();
    if card::is_relate_to_battle(d, at) && !card::is_immune_to_effect(d, at, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        card::register_effect(d, at, e1, false);
        let bt = b_o.nil();
        if card::is_relate_to_battle(d, bt) && !card::is_immune_to_effect(d, bt, a.e) {
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e2, EVENT_BATTLED);
            effect::set_label_object(d, e2, Value::Card(bt));
            effect::set_range(d, e2, LOCATION_MZONE);
            effect::set_operation(d, e2, Some(operation(atkop)));
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
            card::register_effect(d, bt, e2, false);
        }
    }
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let b = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_faceup(d, b) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, b);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, b, e1, false);
    }
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_previous_location(d, c, LOCATION_ONFIELD) {
        let e1 = effect::create_effect(d, c);
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_range(d, e1, LOCATION_GRAVE);
        effect::set_count_limit(d, e1, 1, ID, 0, 0);
        effect::set_target(d, e1, Some(cost_target(sstg)));
        effect::set_operation(d, e1, Some(operation(ssop)));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn sstg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn ssop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, true);
    }
}
