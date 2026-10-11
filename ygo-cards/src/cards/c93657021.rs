//! Destiny HERO - Dusktopia
use super::prelude::*;

pub const ID: u32 = 93657021;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DESTINY_HERO],
    material_setcode: &[SET_HERO, SET_DESTINY_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion materials: 1 "Destiny HERO" Fusion Monster + 1 "Destiny HERO" monster
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), st, Some(tp))
                    && card::is_set_card_ctx(d, c, &[SET_DESTINY_HERO as u16], Some(fc), st, Some(tp))
            })),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_set_card_ctx(d, c, &[SET_DESTINY_HERO as u16], Some(fc), st, Some(tp))
            })),
        ],
    );
    // Fusion Summon 1 Fusion Monster from your Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    let tg = fusion::summon_eff_tg(d, fusion::SummonEff::default());
    effect::set_target(d, e1, Some(tg));
    let op = fusion::summon_eff_op(d, fusion::SummonEff::default());
    effect::set_operation(d, e1, Some(op));
    card::register_effect(d, c, e1, false);
    // Make a monster on the field unable to be destroyed by battle or card effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(indtg)));
    effect::set_operation(d, e2, Some(operation(indop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h) && !duel::is_phase(d, PHASE_DAMAGE)
}
fn indtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn indop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // Cannot be destroyed by battle or card effects
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, 3008);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        card::register_effect(d, tc, e2, false);
        // Neither player takes any battle damage from attacks involving it
        let e3 = effect::clone(d, e1);
        effect::set_description(d, e3, 3213);
        effect::set_code(d, e3, EFFECT_NO_BATTLE_DAMAGE);
        card::register_effect(d, tc, e3, false);
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_AVOID_BATTLE_DAMAGE);
        card::register_effect(d, tc, e4, false);
    }
}
