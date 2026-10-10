//! Tricky Spell 4
use super::prelude::*;

pub const ID: u32 = 75622824;
const TOKEN_TRICKY: u32 = ID + 1;
const CARD_THE_TRICKY: u32 = 14778250;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_THE_TRICKY, TOKEN_TRICKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 face-up "The Tricky" you control to the GY; Special Summon "Tricky Tokens" (Spellcaster-Type/WIND/Level 5/ATK 2000/DEF 1200) in Defense Position, equal to the number of monsters your opponent controls. "Tricky Tokens" cannot declare an attack
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn costfilter(d: &mut Duel, c: CardId, tp: u8, token_count: i32) -> bool {
    card::is_code(d, c, &[CARD_THE_TRICKY])
        && card::is_faceup(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 >= token_count
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    effect::set_label(d, a.e, &[-100]);
    let token_count = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
    if a.chk == Some(0) {
        return token_count > 0 && duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, tp, token_count)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, tp, token_count)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: None,
        type_: Some(TYPES_TOKEN),
        attack: Some(2000),
        defense: Some(1200),
        level: Some(5),
        race: Some(RACE_SPELLCASTER),
        attribute: Some(ATTRIBUTE_WIND),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let token_count = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
    if a.chk == Some(0) {
        let cost_skip_chk = effect::get_label(d, a.e) == -100 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= token_count;
        effect::set_label(d, a.e, &[0]);
        if token_count >= 2 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return false;
        }
        return cost_skip_chk && duel::is_player_can_special_summon_monster(d, tp, TOKEN_TRICKY, token_data(), None, None, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, token_count as u8, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, token_count as u8, tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let token_count = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
    if token_count == 0
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < token_count
        || (token_count >= 2 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some())
        || !duel::is_player_can_special_summon_monster(d, tp, TOKEN_TRICKY, token_data(), None, None, 0)
    {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    for _i in 1..=token_count {
        let token = duel::create_token(d, tp, TOKEN_TRICKY).nil();
        if duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // "Tricky Tokens" cannot declare an attack
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, 3206);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, token, e1, true);
        }
    }
    duel::special_summon_complete(d);
}
