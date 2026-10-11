//! Ritual Beast Ulti-Pettlephin
use super::prelude::*;

pub const ID: u32 = 12678870;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RITUAL_BEAST_TAMER, SET_SPIRITUAL_BEAST],
    material_setcode: &[SET_RITUAL_BEAST, SET_RITUAL_BEAST_TAMER, SET_SPIRITUAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_RITUAL_BEAST_TAMER),aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SPIRITUAL_BEAST))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
                card::is_set_card_ctx(d, x, &[SET_RITUAL_BEAST_TAMER as u16], Some(fc), st, Some(tp))
            })),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
                card::is_set_card_ctx(d, x, &[SET_SPIRITUAL_BEAST as u16], Some(fc), st, Some(tp))
            })),
        ],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,aux.FALSE)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(aux_false())),
            ..Default::default()
        },
    );
    // indes
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // spsummon
    let e4 = effect::create_effect(d, c);
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e4, 0, Some(TIMING_END_PHASE));
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_MZONE, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_extra_as_cost(d, c);
    }
    duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKTOP as i32, REASON_COST, None);
    true
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_RITUAL_BEAST_TAMER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && duel::is_existing_target(d, Some(&|d, cc| filter2(d, cc, e, tp)), tp, LOCATION_REMOVED, 0, 1, c.into())
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SPIRITUAL_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        return ft > 1
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let first = group::get_first(d, &g1).nil();
    let g2 = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, first.into()).nil();
    let mut g1 = g1;
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), 2, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let mut g = duel::get_target_cards(d, a.e).nil();
    if g.is_empty() {
        return;
    }
    if g.len() as i32 <= ft {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, ft as u16, ft as u16, false, NO_EX).nil();
        duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        group::remove_card(&mut g, &sg);
        duel::send_to_grave(d, g, REASON_RULE, None, None);
    }
}
