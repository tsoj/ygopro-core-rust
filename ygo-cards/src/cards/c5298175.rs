//! Fortune Fairy Swee
use super::prelude::*;

pub const ID: u32 = 5298175;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_DRAW);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // banish
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_DELAY | EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return !card::is_public(d, c);
    }
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_HAND)
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && rmfilter(d, chkc) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&rmfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        let pos = card::get_position(d, tc) as u8;
        if duel::remove(d, tc, pos, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
            effect::set_range(d, e1, LOCATION_REMOVED);
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            let label = if duel::is_turn_player(d, a.tp) {
                duel::get_turn_count(d, None) + 2
            } else {
                duel::get_turn_count(d, None) + 1
            };
            effect::set_label(d, e1, &[label as i64]);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_condition(d, e1, Some(condition(retcon)));
            effect::set_operation(d, e1, Some(operation(retop)));
            card::register_effect(d, tc, e1, false);
        }
    }
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_turn_count(d, None) as i64 == effect::get_label(d, a.e)
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::return_to_field(d, c, None, None);
    effect::reset(d, a.e);
}
