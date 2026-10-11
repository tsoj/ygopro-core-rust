//! Cynet Crosswipe
use super::prelude::*;

pub const ID: u32 = 77449773;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.spcheck(sg,tp,exg,dg)`
fn spcheck(d: &mut Duel, sg: &CardSet, dg: &CardSet) -> bool {
    let mut a: i64 = 0;
    for c in group::iter(d, sg) {
        if group::is_contains(dg, c) {
            a += 1;
        }
        let eg = card::get_equip_group(d, c);
        for tc in group::iter(d, &eg) {
            if group::is_contains(dg, tc) {
                a += 1;
            }
        }
    }
    dg.len() as i64 - a >= 1
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_CYBERSE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && chkc != h;
    }
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_can_be_effect_target(d, c, Some(a.e))), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) == 1 {
            effect::set_label(d, a.e, &[0]);
            return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&|d, sg, _tp, _exg| (spcheck(d, sg, &dg), false)), NO_EX);
        } else {
            return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
        }
    }
    if effect::get_label(d, a.e) == 1 {
        effect::set_label(d, a.e, &[0]);
        let sg = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&|d, sg, _tp, _exg| (spcheck(d, sg, &dg), false)), NO_EX);
        duel::release(d, sg, REASON_COST, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into());
    let n = g.clone().nil().len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, n as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
