//! Shield Worm
use super::prelude::*;

pub const ID: u32 = 15939448;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // deckdes
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_position(d, h, POS_FACEUP_ATTACK)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT)), a.tp, LOCATION_MZONE, 0, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, ct as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_position(d, c, POS_FACEUP_ATTACK) {
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT)), a.tp, LOCATION_MZONE, 0, NO_EX);
        duel::discard_deck(d, 1 - a.tp, ct as u16, REASON_EFFECT);
    }
}
