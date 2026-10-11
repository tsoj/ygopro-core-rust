//! D/D/D Alfred the Divine Sage King
//! Scripted by The Razgriz
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 11852093;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DD, SET_DDD, SET_DARK_CONTRACT],
    material_setcode: &[SET_DD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 "D/D" monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DD),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DD as u16], Some(fc), st, Some(tp)))), 2)],
    );

    // Fusion Summon 1 "D/D/D" Fusion Monster from your Extra Deck, by shuffling its materials from your hand, field, and/or banishment into the Deck
    let fusion_params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DDD as u16]))),
        matfilter: Some(Arc::new(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c))),
        extrafil: Some(Arc::new(|d, _e, tp, _mg| {
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)), tp, LOCATION_REMOVED, 0, NO_EX);
            (Some(g), None)
        })),
        extraop: Some(fusion::shuffle_material()),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    };
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    let tg = fusion::summon_eff_tg(d, fusion_params.clone());
    effect::set_target(d, e1, Some(tg));
    let op = fusion::summon_eff_op(d, fusion_params);
    effect::set_operation(d, e1, Some(op));
    card::register_effect(d, c, e1, false);

    // Place "Dark Contract" Spell/Traps face-up on your field from your GY and/or banishment, up to the number of "D/D/D" monsters you control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(pltg)));
    effect::set_operation(d, e2, Some(operation(plop)));
    card::register_effect(d, c, e2, false);
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_REMOVED) as i32);
    true
}
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])
        && card::is_continuous_spell_trap(d, c)
        && card::is_faceup(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
}
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && plfilter(d, chkc, tp);
    }
    let mzone_d = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DDD as u16])), tp, LOCATION_MZONE, 0, NO_EX) as i64;
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 as i64;
    if a.chk == Some(0) {
        return mzone_d > 0 && ft > 0 && duel::is_existing_target(d, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    let max = mzone_d.min(ft).max(0) as u16;
    let g = duel::select_target(d, tp, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, max, false, NO_EX).nil();
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    }
    true
}
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut tg = duel::get_target_cards(d, a.e).nil();
    if tg.is_empty() {
        return;
    }
    let mut gg = CardSet::new();
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    if (ft as i64) < tg.len() as i64 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        let sg = group::filter_select(d, &tg, tp, &|d, c| plfilter(d, c, tp), ft as u16, ft as u16, false, NO_EX).nil();
        gg = tg.clone();
        group::remove_card(&mut gg, &sg);
        tg = sg;
    }
    for tc in group::iter(d, &tg) {
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
    if !gg.is_empty() {
        duel::send_to_grave(d, gg, REASON_RULE | REASON_RETURN, Some(PLAYER_NONE as u8), Some(tp));
    }
}
