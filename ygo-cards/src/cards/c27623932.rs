//! Thunder Discharge
use super::prelude::*;

pub const ID: u32 = 27623932;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ADVENTURER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy monsters your opponent controls and equip a card to 1 monster you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_EQUIP);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[TOKEN_ADVENTURER]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !card::is_faceup(d, c) {
        return false;
    }
    let eg = card::get_equip_group(d, c);
    if !group::is_exists(d, &eg, &|d, c| card::lists_code(d, c, &[TOKEN_ADVENTURER]), 1, NO_EX) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| desfilter(d, cc, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
fn desfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    card::is_faceup(d, c) && card::is_attack_below(d, c, atk)
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let sel = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    let atk = card::get_attack(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, atk)), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 1 - a.tp, LOCATION_MZONE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn eqfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    card::is_faceup(d, c) && card::check_equip_target(d, tc, c)
}
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_equip_spell(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && card::lists_code(d, c, &[TOKEN_ADVENTURER])
        && !card::is_forbidden(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| eqfilter2(d, cc, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    let atk = card::get_attack(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, atk)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty()
        && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| eqfilter(d, c, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        let sel = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| eqfilter(d, c, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let Some(ec) = group::get_first(d, &sel) else { return };
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        let tg = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter2(d, c, ec)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        if tg.is_empty() {
            return;
        }
        duel::hint_selection(d, tg.clone(), true);
        let ectg = group::get_first(d, &tg).nil();
        duel::break_effect(d);
        duel::equip(d, a.tp, ec, ectg, true, false);
    }
}
