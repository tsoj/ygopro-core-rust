//! Lunalight Liger Dancer
use super::prelude::*;

pub const ID: u32 = 54701958;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_LUNALIGHT],
    material_setcode: &[SET_LUNALIGHT],
    listed_names: &[24550676],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Lunalight Leo Dancer" + 3 "Lunalight" monsters
    fusion::add_proc_mix_n(
        d,
        c,
        false,
        false,
        vec![
            (fusion::Mat::Code(24550676), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_LUNALIGHT as u16], Some(fc), st, Some(tp)))), 3),
        ],
    );
    card::add_must_be_fusion_summoned(d, c);
    // Unaffected by card effects, except "Lunalight" cards
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(immune_value));
    card::register_effect(d, c, e1, false);
    // Can make a second attack during each Battle Phase
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // Destroy all your opponent's Special Summoned monsters
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e3, Some(cost_target(descost)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,te) return not te:IsCardSetcode(SET_LUNALIGHT) end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    !effect::is_card_setcode(d, te, &[SET_LUNALIGHT as u16])
}
fn descostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_able_to_grave_as_cost(d, c)
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&descostfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&descostfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_special_summoned(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_special_summoned(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
