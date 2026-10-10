//! Glow-Up Bloom
use super::prelude::*;

pub const ID: u32 = 92964816;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_banish()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[4064256],
    ..ScriptInfo::DEFAULT
};

fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, chk: bool) -> bool {
    card::is_level_above(d, c, 5)
        && card::is_race(d, c, RACE_ZOMBIE)
        && (card::is_able_to_hand(d, c, None)
            || (chk
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn fieldcond(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[4064256])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let ck = duel::is_existing_matching_card(d, Some(&fieldcond), a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp, ck)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, a.tp);
    let ck = duel::is_existing_matching_card(d, Some(&fieldcond), a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, ck)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        if ck
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), None, None)
            && (!card::is_able_to_hand(d, tc, None) || duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)))
        {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        } else {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - a.tp, tc);
        }
    }
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_race(d, c, RACE_ZOMBIE)
}
