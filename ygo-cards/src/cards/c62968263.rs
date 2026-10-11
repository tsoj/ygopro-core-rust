//! Galaxy-Eyes Afterglow Dragon
use super::prelude::*;

pub const ID: u32 = 62968263;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_GALAXYEYES_P_DRAGON],
    listed_series: &[SET_GALAXY_EYES, SET_NUMBER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Special Summon or attach a "Galaxy-Eyes Photon Dragon"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(atcon)));
    effect::set_target(d, e2, Some(cost_target(attg)));
    effect::set_operation(d, e2, Some(operation(atop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_REMOVE);
    card::register_effect(d, c, e3, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GALAXY_EYES as u16])),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_COST)
        && effect::is_activated(d, a.re.nil())
        && effect::is_active_type(d, a.re.nil(), TYPE_XYZ)
        && card::is_previous_location(d, c, LOCATION_OVERLAY)
}
fn gepdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON])
        && ((ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
            || duel::is_existing_matching_card(d, Some(&|d, x| attfilter(d, x, e)), tp, LOCATION_MZONE, 0, 1, NO_EX))
}
fn attfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && !card::is_immune_to_effect(d, c, e)
}
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| gepdfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    let label = if duel::is_battle_phase(d, None) { 1 } else { 0 };
    effect::set_label(d, a.e, &[label]);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
fn numbfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_NUMBER as u16])
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| gepdfilter(d, c, e, tp, ft)),
        tp,
        LOCATION_DECK | LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let Some(tc) = group::get_first(d, &sel) else {
        return;
    };
    let spchk = ft > 0 && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None);
    let attchk = duel::is_existing_matching_card(d, Some(&|d, c| attfilter(d, c, e)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    if !(spchk || attchk) {
        return;
    }
    let op = duel::select_effect(d, tp, &[(spchk, aux::stringid(ID, 3)), (attchk, aux::stringid(ID, 4))]);
    let mut success_chk = false;
    if op == Some(1) && duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        success_chk = true;
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        let osel = duel::select_matching_card(
            d,
            tp,
            Some(&|d, c| attfilter(d, c, e)),
            tp,
            LOCATION_MZONE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        if let Some(oc) = group::get_first(d, &osel) {
            success_chk = true;
            duel::hint_selection(d, oc, true);
            duel::overlay(d, oc, tc, false);
        }
    }
    let ng = duel::get_matching_group(d, Some(&|d, c| numbfilter(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    if success_chk && effect::get_label(d, e) == 1 && !ng.is_empty() {
        let c = effect::get_handler(d, e).nil();
        duel::break_effect(d);
        for sc in group::iter(d, &ng) {
            //Increase ATK
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            let atk = card::get_attack(d, sc) as i64 * 2;
            effect::set_value(d, e1, atk);
            card::register_effect(d, sc, e1, false);
        }
    }
}
