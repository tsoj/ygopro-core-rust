//! The First Darklord
use super::prelude::*;

pub const ID: u32 = 4167084;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[25451652], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion materials
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| ffilter(d, x, Some(fc), st, tp))), 3)]);
    // Destroy opponent's field
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_category(d, e0, CATEGORY_DESTROY);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e0, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e0, 1, ID, 0, 0);
    effect::set_label(d, e0, &[0]);
    effect::set_condition(d, e0, Some(condition(descon)));
    effect::set_target(d, e0, Some(cost_target(destg)));
    effect::set_operation(d, e0, Some(operation(desop)));
    card::register_effect(d, c, e0, false);
    // If fusion summoned with "Darklord Morningstar"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e1, effect_card_value(valcheck));
    effect::set_label_object(d, e1, Value::Effect(e0));
    card::register_effect(d, c, e1, false);
    // Opponent cannot target your fairy monsters
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_FAIRY))));
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
    // Special summon 1 fairy monster from hand/GY
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END));
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn ffilter(d: &mut Duel, c: CardId, fc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, fc, sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_FAIRY, fc, sumtype, Some(tp))
}
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    if group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[25451652]), 1, NO_EX) {
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    }
    0
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
    effect::set_label(d, a.e, &[0]);
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
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_FAIRY) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
