//! Trap of the Poisonous Scorpion
use super::prelude::*;

pub const ID: u32 = 212652;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy an opponent's attacking monster, then inflict 300 damage to your opponent
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let atk = duel::get_attacker(d).nil();
    card::is_controler(d, atk, 1 - a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let bc = duel::get_attacker(d).nil();
    effect::set_label_object(d, a.e, Value::Card(bc));
    card::create_effect_relation(d, bc, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(bc)), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 300);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_effect(d, bc, a.e) && card::is_monster(d, bc) && duel::destroy(d, bc, REASON_EFFECT, None, None) > 0 {
        duel::break_effect(d);
        duel::damage(d, 1 - a.tp, 300, REASON_EFFECT, false, None);
    }
}
