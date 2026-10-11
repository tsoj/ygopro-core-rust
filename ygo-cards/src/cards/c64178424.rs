//! Will of the Salamangreat
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 64178424;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.EnableCheckReincarnation(c)
    aux::enable_check_reincarnation(d, c);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);

    // spsummon (single)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg1)));
    effect::set_operation(d, e2, Some(operation(spop1)));
    card::register_effect(d, c, e2, false);

    // spsummon (link)
    let e3 = effect::clone(d, e2);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(sptg2)));
    effect::set_operation(d, e3, Some(operation(spop2)));
    card::register_effect(d, c, e3, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SALAMANGREAT],
    ..ScriptInfo::DEFAULT
};

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, pos: u32) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(pos as u8), None, None)
}

fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, POS_FACEUP)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}

fn spop1(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp, POS_FACEUP));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}

fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_monster(d, c) && card::is_reincarnation_summoned(d, c)
}

fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, POS_FACEUP_DEFENSE)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}

fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    let loc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let link = card::get_link(d, tc) as i64;
    let mut ft = loc.min(link);
    if ft < 1 {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp, POS_FACEUP_DEFENSE));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, ft as u16, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
