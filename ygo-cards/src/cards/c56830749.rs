//! Share the Pain
use super::prelude::*;

pub const ID: u32 = 56830749;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make the opponent tribute a monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    duel::check_release_group(d, 1 - tp, None, 1, duel::ReleaseArgs::default(), c.into(), false)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group(d, 1 - a.tp, None, 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_RELEASE as u64);
    let g = duel::select_release_group(d, 1 - tp, None, 1, 1, duel::ReleaseArgs::default(), NO_EX, false);
    if let Some(g) = g {
        duel::release(d, g, REASON_RULE, Some(1 - tp));
    }
}
