//! Number 96: Dark Mist
use super::prelude::*;

pub const ID: u32 = 55727845;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(96), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(2), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // atk u/d
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let at = duel::get_attack_target(d);
    if a.chk == Some(0) {
        let left = match at {
            Some(at) => card::is_faceup(d, at) && duel::get_attacker(d) == Some(c),
            None => false,
        };
        return (left || at == Some(c)) && !card::is_status(d, c, STATUS_CHAINING);
    }
    let bt = card::get_battle_target(d, c);
    duel::set_target_card(d, bt.nil());
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
            let atk = lua_int(card::get_attack(d, tc) as f64 / 2.0);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            effect::set_value(d, e1, atk);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_range(d, e2, LOCATION_MZONE);
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e2, atk);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e2, false);
        }
    }
}
