//! Moisture Creature
use super::prelude::*;

pub const ID: u32 = 75285069;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 3 tribute
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE + 1), Some(aux::stringid(ID, 0)), None, None);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_TRIBUTE+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_TRIBUTE + 1
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
