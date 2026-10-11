//! Dandy Whitelion
use super::prelude::*;

pub const ID: u32 = 36629635;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[36629636], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 3 "White Fluff Token"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(tkcost)));
    effect::set_target(d, e1, Some(cost_target(tktg)));
    effect::set_operation(d, e1, Some(operation(tkop)));
    card::register_effect(d, c, e1, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_HAND | LOCATION_ONFIELD)
}
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    !card::is_summon_location(d, a.c(0), LOCATION_EXTRA)
}
fn tkcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // Cannot Special Summon monsters from the Extra Deck
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_PLANT),
        attribute: Some(ATTRIBUTE_WIND),
    }
}
fn can_summon_tokens(d: &mut Duel, tp: u8) -> bool {
    !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 3
        && duel::is_player_can_special_summon_monster(d, tp, 36629636, token_data(), Some(POS_FACEUP_DEFENSE as u8), None, 0)
}
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return can_summon_tokens(d, a.tp);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 3, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, a.tp, 0);
    true
}
fn tkop(d: &mut Duel, a: &EffArgs) {
    if can_summon_tokens(d, a.tp) {
        for _ in 0..3 {
            let token = duel::create_token(d, a.tp, 36629636).nil();
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
        duel::special_summon_complete(d);
    }
}
