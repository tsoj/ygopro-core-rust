//! Banquet of Millions
use super::prelude::*;

pub const ID: u32 = 48814566;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
}
fn able_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
fn able_remove_facedown(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && card::is_facedown(d, c)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&able_remove), tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&able_remove_facedown), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 2, 0, LOCATION_EXTRA as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&able_remove), tp, LOCATION_EXTRA, 0, NO_EX);
    let og = duel::get_matching_group(d, Some(&able_remove_facedown), tp, 0, LOCATION_EXTRA, NO_EX);
    if g.is_empty() || og.is_empty() {
        return;
    }
    let ct = g.len().min(og.len());
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let rg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
    let rmct = duel::remove(d, rg, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    let rmog = duel::get_operated_group(d);
    let removed_here = group::filter_count(d, &rmog, &|d, x| card::is_location(d, x, LOCATION_REMOVED), NO_EX);
    if rmct > 0 && rmct as usize == removed_here {
        let org = group::random_select(d, &og, tp, rmct as usize);
        if duel::remove(d, org, POS_FACEUP as u8, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0 {
            let fid = card::get_field_id(d, c);
            let opg = duel::get_operated_group(d);
            for oc in group::iter(d, &opg) {
                card::register_flag_effect(d, oc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
            }
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            effect::set_label(d, e1, &[fid as i64]);
            effect::set_label_object(d, e1, Value::Group(opg));
            effect::set_condition(d, e1, Some(condition(retcon)));
            effect::set_operation(d, e1, Some(operation(retop)));
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e1, tp);
        }
    }
}
fn retfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    if !group::is_exists(d, &g, &|d, c| retfilter(d, c, fid), 1, NO_EX) {
        effect::reset(d, a.e);
        return false;
    }
    true
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    let sg = group::filter(d, &g, &|d, c| retfilter(d, c, fid), NO_EX);
    duel::send_to_deck(d, &sg, Some(1 - tp), SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_RETURN, None);
}
