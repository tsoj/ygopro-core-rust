//! Tsukuyomi
use super::prelude::*;

pub const ID: u32 = 34853266;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Cannot be Special Summoned
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // Flip 1 monster face-down
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(postg)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_FLIP);
    card::register_effect(d, c, e3, false);
}
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_turn_set(d, c)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && posfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, a.tp, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n as u8, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
