//! Quintet Magician
use super::prelude::*;

pub const ID: u32 = 84433295;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    // Fusion.AddProcFunRep(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),5,true)
    fusion::add_proc_fun_rep(d, c, fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(fc), st, Some(tp))), 5, true);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux::fuslimit());
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e3, lua_fn(valcheck));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
    // cannot release
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_UNRELEASABLE_SUM);
    effect::set_value(d, e4, 1);
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_UNRELEASABLE_NONSUM);
    card::register_effect(d, c, e5, false);
    // cannot be fusion material
    let e6 = effect::clone(d, e4);
    effect::set_code(d, e6, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    card::register_effect(d, c, e6, false);
    // indes
    let e7 = effect::clone(d, e4);
    effect::set_code(d, e7, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e7, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let (e, c) = (a.e(0), a.c(1));
    let mat = card::get_material(d, c);
    let g = group::filter(d, &mat, &|d, x| card::is_race(d, x, RACE_SPELLCASTER), NO_EX);
    if group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) == 5 {
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h) && effect::get_label(d, a.e) == 1
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
