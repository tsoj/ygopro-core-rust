//! Performapal Odd-Eyes Metal Claw
use super::prelude::*;

pub const ID: u32 = 65029288;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ODD_EYES, SET_PERFORMAPAL],
    listed_names: &[CARD_POLYMERIZATION],
    material_setcode: &[SET_ODD_EYES, SET_PERFORMAPAL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ODD_EYES),aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PERFORMAPAL))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ODD_EYES as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_PERFORMAPAL as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // cannot fusion material
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // indes
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(immcon)));
    effect::set_operation(d, e2, Some(operation(immop)));
    card::register_effect(d, c, e2, false);
    // atkup
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(re) = a.re else {
        return false;
    };
    let rh = effect::get_handler(d, re).nil();
    if !card::is_code(d, rh, &[CARD_POLYMERIZATION]) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
fn immop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(efilter));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 300);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
        card::register_effect(d, tc, e1, false);
    }
}
