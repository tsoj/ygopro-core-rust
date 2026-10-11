//! Number C65: King Overfiend
use super::prelude::*;

pub const ID: u32 = 49195710;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(65), listed_names: &[3790062], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 3 DARK monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK),3,3)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, sc, st, Some(tp)))),
        Some(3),
        3,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Make 1 face-up monster your opponent controls lose 1000 ATK and DEF
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(atkdeftg)));
    effect::set_operation(d, e1, Some(operation(atkdefop)));
    card::register_effect(d, c, e1, false);
    // Effect Monsters your opponent controls cannot activate their effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(cond2)));
    card::register_effect(d, c, e2, false);
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[3790062]), 1, NO_EX)
}
fn atkdeftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g.clone(), 1, tp, -1000);
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, g, 1, tp, -1000);
    true
}
fn atkdefop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let c = effect::get_handler(d, a.e).nil();
        // It loses 1000 ATK and DEF
        card::update_attack(d, tc, -1000, None, Some(c), None);
        card::update_defense(d, tc, -1000, None, Some(c), None);
    }
}
