//! Snowdrop the Rikka Fairy
use super::prelude::*;

pub const ID: u32 = 33491462;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RIKKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute 1 Plant; Special Summon this card and 1 other plant from hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Level change
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    effect::set_operation(d, e2, Some(operation(lvop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    aux::is_rikka_releasable(d, c, tp) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 1
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`: the filter gets `tp` as its extra argument.
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_PLANT) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, c.into())
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, c.into());
        if let Some(mut g) = g {
            if !g.is_empty() {
                group::add_card(&mut g, &CardSet::from_card(c));
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_PLANT))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
/// `s.lvfilter1(c,tp)`
fn lvfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let lv = card::get_level(d, c);
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_PLANT)
        && card::has_level(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| lvfilter2(d, x, lv)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.lvfilter2(c,lv)`
fn lvfilter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT) && card::is_level_above(d, c, 0) && card::get_level(d, c) != lv
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && lvfilter1(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| lvfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let _g = duel::select_target(d, tp, Some(&|d, c| lvfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) {
        return;
    }
    let lv = card::get_level(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, c| lvfilter2(d, c, lv)), tp, LOCATION_MZONE, 0, NO_EX);
    let c = effect::get_handler(d, e).nil();
    for lc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        effect::set_value(d, e1, lv as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, lc, e1, false);
    }
}
