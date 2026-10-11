//! BF - Etesian of Two Swords
use super::prelude::*;

pub const ID: u32 = 78564023;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Inflict 1000 Damage to your opponent
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let atk = duel::get_attacker(d);
    let tgt = duel::get_attack_target(d);
    match (atk, tgt) {
        (Some(atk), Some(tgt)) => {
            (card::is_controler(d, atk, tp) && card::is_set_card(d, atk, &[SET_BLACKWING as u16]) && card::is_relate_to_battle(d, tgt))
                || (card::is_controler(d, tgt, tp) && card::is_set_card(d, tgt, &[SET_BLACKWING as u16]) && card::is_relate_to_battle(d, atk))
        }
        _ => false,
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let v = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, amount) = (v[0].int() as u8, v[1].int());
    duel::damage(d, p, amount, REASON_EFFECT, false, None);
}
