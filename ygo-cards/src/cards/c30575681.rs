//! Noble Knight Bedwyr
use super::prelude::*;

pub const ID: u32 = 30575681;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NOBLE_ARMS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // send to grave
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // equip change
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_property(d, e3, EFFECT_FLAG_NO_TURN_RESET + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    effect::set_operation(d, e3, Some(operation(eqop)));
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_EQUIP) && card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0 {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
fn eqfilter1(d: &mut Duel, c: CardId) -> bool {
    if !card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]) {
        return false;
    }
    match card::get_equip_target(d, c) {
        Some(et) => duel::is_existing_target(d, Some(&|d, c2| eqfilter2(d, c2, c)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, et.into()),
        None => false,
    }
}
fn eqfilter2(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    card::is_faceup(d, c) && card::check_equip_target(d, ec, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&eqfilter1), tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g1 = duel::select_target(d, tp, Some(&eqfilter1), tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g1).nil();
    effect::set_label_object(d, a.e, Value::Card(tc));
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let ex = card::get_equip_target(d, tc).map_or(NO_EX, |x| x.into());
    let _g2 = duel::select_target(d, tp, Some(&|d, c| eqfilter2(d, c, tc)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, ex);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let ec = effect::get_label_object(d, a.e).as_card().nil();
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).remove(0).as_group().cloned().nil();
    let mut tc = group::get_first(d, &g);
    if tc == Some(ec) {
        tc = group::iter(d, &g).get(1).copied();
    }
    if card::is_faceup(d, ec) && card::is_relate_to_effect(d, ec, a.e) {
        let tc = tc.nil();
        if card::check_equip_target(d, ec, tc) && card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            duel::equip(d, a.tp, ec, tc, true, false);
        }
    }
}
