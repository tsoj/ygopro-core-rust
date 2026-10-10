//! Staysailor Romarin
use super::prelude::*;

pub const ID: u32 = 49964567;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Prevent destruction
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
    // Send 1 Plant monster from Deck/ED to GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(grcon)));
    effect::set_target(d, e2, Some(cost_target(grtg)));
    effect::set_operation(d, e2, Some(operation(grop)));
    card::register_effect(d, c, e2, false);
}
fn trgtfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, c.into())
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| trgtfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let _g = duel::select_target(d, tp, Some(&|d, x| trgtfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_MZONE as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, tc.into()).nil();
            let sent = !g.is_empty() && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) > 0;
            let first = if sent { Some(group::get_first(d, &g).nil()) } else { None };
            if first.is_some_and(|f| card::is_location(d, f, LOCATION_GRAVE)) {
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                effect::set_value_fn(d, e1, lua_fn(indval));
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, tc, e1, true);
            }
        }
    }
}
/// `function(e,re,r,rp) return (r&REASON_BATTLE+REASON_EFFECT)~=0 end`
fn indval(_d: &mut Duel, a: Args) -> bool {
    let r = a.int(2) as u32;
    (r & (REASON_BATTLE + REASON_EFFECT)) != 0
}
fn grcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_EFFECT)
}
fn grfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_below(d, c, 5) && card::has_level(d, c) && card::is_race(d, c, RACE_PLANT) && card::is_able_to_grave(d, c, None)
}
fn grtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&grfilter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
fn grop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&grfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
