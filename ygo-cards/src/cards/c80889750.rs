//! Frightfur Sabre-Tooth
use super::prelude::*;

pub const ID: u32 = 80889750;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FRIGHTFUR],
    material_setcode: &[SET_FLUFFAL, SET_EDGE_IMP, SET_FRIGHTFUR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| mfilter2(d, x, fc, st, tp))),
        1,
        99,
        vec![fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| mfilter1(d, x, fc, st, tp)))],
    );
    // Special summon from the Graveyard
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Increase ATK
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, x| card::is_set_card(d, x, &[SET_FRIGHTFUR as u16]))));
    effect::set_value(d, e3, 400);
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e4, lua_fn(valcheck));
    card::register_effect(d, c, e4, false);
}
fn mfilter1(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_set_card_ctx(d, c, &[SET_FRIGHTFUR as u16], Some(fc), sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), sumtype, Some(tp))
}
fn mfilter2(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_set_card_ctx(d, c, &[SET_FLUFFAL as u16], Some(fc), sumtype, Some(tp))
        || card::is_set_card_ctx(d, c, &[SET_EDGE_IMP as u16], Some(fc), sumtype, Some(tp))
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    (card::get_summon_type(d, h) & SUMMON_TYPE_FUSION) == SUMMON_TYPE_FUSION
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, a: Args) {
    let c = a.c(1);
    if card::get_material_count(d, c) >= 3 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e1, 1);
        effect::set_condition(d, e1, Some(condition(indcon)));
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        card::register_effect(d, c, e1, true);
        let e2 = effect::clone(d, e1);
        effect::set_description(d, e2, aux::stringid(ID, 1));
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        card::register_effect(d, c, e2, true);
    }
}
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
