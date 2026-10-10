//! Lubellion the Searing Dragon
use super::prelude::*;

pub const ID: u32 = 70534340;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon materials: 1 DARK monster + "Fallen of Albaz"
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_ALBAZ),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), st, Some(tp)))),
        ],
    );
    card::enable_revive_limit(d, c);
    // Fusion Summon 1 Level 8 or lower Fusion Monster from your Extra Deck, shuffling Fusion Materials into the Deck, from your monsters on the field, GY, and/or face-up banished cards
    let params = || fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_level_below(d, c, 8) && !card::is_code(d, c, &[ID]))),
        matfilter: Some(fusion::on_field_mat_with(std::sync::Arc::new(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c)))),
        extrafil: Some(std::sync::Arc::new(fextra)),
        extraop: Some(fusion::shuffle_material()),
        stage2: Some(std::sync::Arc::new(stage2)),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    };
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    let tg = fusion::summon_eff_tg(d, params());
    effect::set_target(d, e1, Some(tg));
    let op = fusion::summon_eff_op(d, params());
    effect::set_operation(d, e1, Some(op));
    card::register_effect(d, c, e1, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)),
        tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        NO_EX,
    );
    (Some(g), None)
}
/// `s.stage2(e,tc,tp,mg,chk)`
fn stage2(d: &mut Duel, e: EffectId, _tc: Option<CardId>, tp: u8, _mg: Option<&CardSet>, chk: i64) {
    if chk == 2 {
        let c = effect::get_handler(d, e).nil();
        // Cannot attack
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3206);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
        // Cannot Special Summon from the Extra Deck, except Fusion Monsters
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, aux::stringid(ID, 1));
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_target_range(d, e2, 1, 0);
        effect::set_target(d, e2, Some(lua_fn(splimit)));
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e2, tp);
    }
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    !card::is_type(d, c, TYPE_FUSION) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, (LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
