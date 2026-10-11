//! Amaze Attraction Horror House
use super::prelude::*;

pub const ID: u32 = 92650018;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZEMENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // From cards_specific_functions.lua
    aux::add_attraction_equip_proc(d, c);
    // You: Target 1 Effect Monster your opponent controls; negate its effects until the end of this turn.
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(aux::attraction_equip_con(true)));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // Your opponent: Change the equipped monster to face-down Defense Position.
    let e2 = effect::clone(d, e1);
    effect::set_property(d, e2, 0, 0);
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_condition(d, e2, Some(aux::attraction_equip_con(false)));
    effect::set_target(d, e2, Some(cost_target(postg)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
}
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsFaceup() and not c:IsDisabled() and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && !card::is_disabled(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && disfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc)
        && card::is_relate_to_effect(d, tc, a.e)
        && card::is_monster(d, tc)
        && !card::is_disabled(d, tc)
        && card::is_controler(d, tc, 1 - a.tp)
    {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, c);
    if a.chk == Some(0) {
        return match tc {
            Some(t) => card::is_can_turn_set(d, t),
            None => false,
        };
    }
    duel::set_operation_info(d, 0, CATEGORY_POSITION, tc.map(CardSet::from_card), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_equip_target(d, c) {
        if card::is_location(d, tc, LOCATION_MZONE) && card::is_faceup(d, tc) {
            duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
