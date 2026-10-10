//! Silent Wobby
use super::prelude::*;

pub const ID: u32 = 71015787;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Sumon itself to the opponent's field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Draw 1 card and the opponent gains 2000 LP
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_RECOVER);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
    })));
    effect::set_target(d, e2, Some(cost_target(efftg)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
    // Change the hand size limit
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_HAND_LIMIT);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, 1, 0);
    effect::set_value(d, e3, 3);
    card::register_effect(d, c, e3, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 1, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 1, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - a.tp, 2000);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::draw(d, tp, 1, REASON_EFFECT) > 0 {
        duel::recover(d, 1 - tp, 2000, REASON_EFFECT, false, None);
    }
}
