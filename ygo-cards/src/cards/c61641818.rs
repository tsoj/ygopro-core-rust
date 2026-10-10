//! Virtual World Dragon - Longlong
use super::prelude::*;

pub const ID: u32 = 61641818;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,3,2,nil,nil,Xyz.InfiniteMats,nil,false,s.xyzcheck)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, Some(xyz::INFINITE_MATS), None, false, Some(xyz::ex_chk(xyzcheck)));
    // Cannot be target
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(ntcon)));
    effect::set_value_fn(d, e1, aux::tgoval());
    card::register_effect(d, c, e1, false);
    // Negate
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_NEGATE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    group::get_class_count(d, g, &|d, c| card::get_race(d, c) as i64) == 1
        && group::get_class_count(d, g, &|d, c| card::get_attribute(d, c) as i64) == 1
}
fn ntcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) > 0
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.rp == a.tp || card::is_status(d, h, STATUS_BATTLE_DESTROYED) || !effect::is_monster_effect(d, a.re.nil()) || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return false;
    }
    let rc = effect::get_handler(d, a.re.nil()).nil();
    let attr = card::get_attribute(d, rc);
    !group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, attr), 1, NO_EX)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_activation(d, a.ev as u8);
}
