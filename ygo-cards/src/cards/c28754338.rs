//! Legendary Atlantean Tridon
use super::prelude::*;

pub const ID: u32 = 28754338;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[47826112], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, ft: i64, tp: u8) -> bool {
    card::is_race(d, c, RACE_SEASERPENT) && (ft > 0 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp))) && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let c = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        return ft > -1 && card::is_releasable(d, c, None, None) && duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    let mut rg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, c.into());
    group::add_card(&mut rg, &CardSet::from_card(c));
    duel::release(d, rg, REASON_COST, None);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[47826112]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) == 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        duel::break_effect(d);
        let tg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
        let _ = group::get_first(d, &tg);
        let h = effect::get_handler(d, a.e).nil();
        for tc in group::iter(d, &tg) {
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, -300);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
