//! Blast Sphere
use super::prelude::*;

pub const ID: u32 = 26302522;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    effect::set_condition(d, e1, Some(condition(eqcon)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attack_target(d) == Some(c) && card::get_battle_position(d, c) == POS_FACEDOWN_DEFENSE
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::get_attacker(d).nil();
    if !card::is_relate_to_battle(d, tc) || !card::is_relate_to_battle(d, c) {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 || card::is_facedown(d, tc) {
        duel::destroy(d, c, REASON_EFFECT, None, None);
        return;
    }
    duel::equip(d, tp, c, tc, true, false);
    // Equip limit
    let e1 = effect::create_effect(d, tc);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
    card::register_effect(d, c, e1, false);
    // Destroy and deal damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_OPPO_TURN | RESET_PHASE | PHASE_STANDBY, 1);
    card::register_effect(d, c, e2, false);
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_owner(d, e) == Some(c)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let ec = card::get_equip_target(d, effect::get_handler(d, a.e).nil()).nil();
    card::create_effect_relation(d, ec, a.e);
    effect::set_label_object(d, a.e, Value::Card(ec));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(ec.into()), 1, 0, 0);
    let atk = card::get_attack(d, ec);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let ec = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_effect(d, ec, a.e) && card::is_faceup(d, ec) {
        let atk = card::get_attack(d, ec);
        if duel::destroy(d, ec, REASON_EFFECT, None, None) > 0 {
            duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
