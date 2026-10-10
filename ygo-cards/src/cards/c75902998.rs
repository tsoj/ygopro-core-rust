//! Trap Hole of Spikes
use super::prelude::*;

pub const ID: u32 = 75902998;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp) && card::is_status(d, at, STATUS_SUMMON_TURN + STATUS_SPSUMMON_TURN)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d);
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, at.nil());
    }
    // math.max(at:GetBaseAttack()/2,0)
    let dam = (card::get_base_attack(d, at.nil()) as f64 / 2.0).max(0.0);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at.map(CardSet::from_card), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, lua_int(dam) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let at = duel::get_attacker(d);
    if card::is_relate_to_battle(d, at.nil()) && duel::destroy(d, at.nil(), REASON_EFFECT, None, None) != 0 {
        let atk = card::get_base_attack(d, at.nil()) as f64 / 2.0;
        if atk > 0.0 {
            duel::damage(d, 1 - a.tp, lua_int(atk), REASON_EFFECT, false, None);
        }
    }
}
