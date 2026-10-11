//! Advanced Heraldry Art
use super::prelude::*;

pub const ID: u32 = 61314842;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERALDIC_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 "Heraldic Beast" monsters from your GY and Xyz Summon using them as material
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_HERALDIC_BEAST as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.xyzfilter(c,mg,tp,chk)`: `tp` and `chk` are only given by the call in `activate`.
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet, tp: u8, chk: bool) -> bool {
    card::is_xyz_summonable(d, c, None, Some(mg.clone()), 2, 2)
        && (!chk || duel::get_location_count_from_ex(d, tp, Some(tp), mg.into(), duel::ExtraCard::Card(c), None).0 > 0)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chkc().is_some() {
        return false;
    }
    let mg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        return mg.len() >= 2
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| xyzfilter(d, c, &mg, tp, false)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    let exg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &mg, tp, false)), tp, LOCATION_EXTRA, 0, NO_EX);
    // s.rescon(exg)
    let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        (
            sg.len() == 2 && group::is_exists(d, &exg, &|d, c| card::is_xyz_summonable(d, c, None, Some(sg.clone()), 2, 2), 1, NO_EX),
            false,
        )
    };
    let sg = aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, sg.clone());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), 2, 0, 0);
    true
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    let mut g = duel::get_target_cards(d, e).nil();
    group::match_(d, &mut g, &|d, c| filter2(d, c, e, tp), NO_EX);
    if g.len() != 2 {
        return;
    }
    if duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) != 2 {
        return;
    }
    duel::break_effect(d);
    let xyzg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &g, tp, true)), tp, LOCATION_EXTRA, 0, NO_EX);
    if !xyzg.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sel = group::select(d, &xyzg, tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        duel::xyz_summon(d, tp, xyz, Some(g), None, 2, 2);
    }
}
