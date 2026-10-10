//! Ghost Ogre & Snow Rabbit
use super::prelude::*;

pub const ID: u32 = 59438930;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // double
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler():IsOnField() and (re:IsMonsterEffect()
    //     or (re:IsSpellTrapEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE)))
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    card::is_on_field(d, rh)
        && (effect::is_monster_effect(d, re) || (effect::is_spell_trap_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        return card::is_destructable(d, rh, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
