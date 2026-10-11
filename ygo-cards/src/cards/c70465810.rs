//! Blackwing - Sudri the Phantom Glimmer
use super::prelude::*;

pub const ID: u32 = 70465810;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_BLACK_WINGED_DRAGON, ID + 1], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 card that lists "Black-Winged Dragon"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Illusory Spark Token"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA) || card::is_type(d, c, TYPE_SYNCHRO)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::lists_code(d, c, &[CARD_BLACK_WINGED_DRAGON]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0
            && duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    let h = effect::get_handler(d, e).nil();
    aux::add_temp_lizard_check(d, h, tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
    true
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_type(d, c, TYPE_SYNCHRO) && card::is_location(d, c, LOCATION_EXTRA)
}
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_original_type(d, c, TYPE_SYNCHRO)
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN | TYPE_TUNER),
        attack: Some(700),
        defense: Some(700),
        level: Some(2),
        race: Some(RACE_WINGEDBEAST),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, tp, 700);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    if !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    let token = duel::create_token(d, tp, ID + 1).nil();
    if duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        duel::break_effect(d);
        duel::damage(d, tp, 700, REASON_EFFECT, false, None);
    }
}
