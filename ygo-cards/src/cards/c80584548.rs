//! Ballista Squad
use super::prelude::*;

pub const ID: u32 = 80584548;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.spcheck(sg, tp, exg, dg)`: a card of `dg` is left to target (`dg` is the extra argument)
fn spcheck(d: &mut Duel, sg: &CardSet, dg: &CardSet) -> bool {
    let mut a: i64 = 0;
    for c in group::iter(d, sg) {
        if group::is_contains(dg, c) {
            a += 1;
        }
        let eq = card::get_equip_group(d, c);
        for tc in group::iter(d, &eq) {
            if group::is_contains(dg, tc) {
                a += 1;
            }
        }
    }
    dg.len() as i64 - a >= 1
}
/// `spcheck` as a release check: the Lua function returns one value, so `stop` is false
fn spcheck_rel(d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet, dg: &CardSet) -> (bool, bool) {
    (spcheck(d, sg, dg), false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, e).nil();
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && chkc != h;
    }
    let h = effect::get_handler(d, e).nil();
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_can_be_effect_target(d, c, Some(e))), tp, 0, LOCATION_ONFIELD, h.into());
    if a.chk == Some(0) {
        if effect::get_label(d, e) == 1 {
            effect::set_label(d, e, &[0]);
            // s.cfilter is nil in this script: no filter
            return duel::check_release_group_cost(d, tp, None, 1, 1, false, Some(&|d: &mut Duel, sg: &CardSet, p: u8, exg: &CardSet| spcheck_rel(d, sg, p, exg, &dg)), NO_EX);
        } else {
            return duel::is_existing_target(d, Some(&|_, _| true), tp, 0, LOCATION_ONFIELD, 1, h.into());
        }
    }
    if effect::get_label(d, e) == 1 {
        effect::set_label(d, e, &[0]);
        // s.cfilter is nil in this script: no filter
        let sg = duel::select_release_group_cost(d, tp, None, 1, 1, false, Some(&|d: &mut Duel, sg: &CardSet, p: u8, exg: &CardSet| spcheck_rel(d, sg, p, exg, &dg)), NO_EX);
        duel::release(d, sg, REASON_COST, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&|_, _| true), tp, 0, LOCATION_ONFIELD, 1, 1, false, h.into()).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
