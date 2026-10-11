//! Thousand Needles
use super::prelude::*;

pub const ID: u32 = 33977496;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.GetAttackTarget()==c and ((c:IsOnField() and c:IsPosition(POS_DEFENSE)) or c:IsPreviousPosition(POS_DEFENSE))
    //   and Duel.GetAttacker():GetAttack()<c:GetDefense()
    duel::get_attack_target(d) == Some(c)
        && ((card::is_on_field(d, c) && card::is_position(d, c, POS_DEFENSE as u32)) || card::is_previous_position(d, c, POS_DEFENSE as u32))
        && card::get_attack(d, duel::get_attacker(d).nil()) < card::get_defense(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,0,0)
    let attacker = duel::get_attacker(d);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, attacker.map(CardSet::from_card), 1, 0, 0);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d).nil();
    // if not a:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, a) {
        return;
    }
    // Duel.Destroy(a,REASON_EFFECT)
    duel::destroy(d, a, REASON_EFFECT, None, None);
}
