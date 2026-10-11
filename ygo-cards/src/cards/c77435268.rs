//! Checkker
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 77435268;
const TOKEN_IRON_RUBBLE: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_IRON_RUBBLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be Normal Summoned if you control a monster
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_CANNOT_SUMMON);
    effect::set_condition(d, e0, Some(condition(e0cond)));
    card::register_effect(d, c, e0, false);
    // Tribute: Special Summon 1 Machine monster from your Deck whose ATK equals its own DEF
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(spfromdecktg)));
    effect::set_operation(d, e1, Some(operation(spfromdeckop)));
    card::register_effect(d, c, e1, false);
    // Banish this card from your GY and discard 1 card; Special Summon 1 "Iron Rubble Token"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::and(vec![cost::self_banish(), cost::discard(None, false, None, None, None)])));
    effect::set_target(d, e2, Some(cost_target(tokensptg)));
    effect::set_operation(d, e2, Some(operation(tokenspop)));
    card::register_effect(d, c, e2, false);
}
fn e0cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) > 0
}
fn spfromdeckfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_MACHINE) && {
        let atk = card::get_attack(d, c);
        card::is_defense(d, c, &[atk as u32])
    } && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn spfromdecktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfromdeckfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn spfromdeckop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfromdeckfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Return it to the hand during the End Phase
        let oper: aux::DelayedFn = std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
            duel::send_to_hand(d, ag, None, REASON_EFFECT, None);
            true
        });
        aux::delayed_operation(d, &g, PHASE_END, ID, a.e, tp, Some(oper), None, Some(0), Some(0), Some(aux::stringid(ID, 2)), None);
    }
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn tokensptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, TOKEN_IRON_RUBBLE, token_data(), None, None, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
fn tokenspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_IRON_RUBBLE, token_data(), None, None, 0)
    {
        let token = duel::create_token(d, tp, TOKEN_IRON_RUBBLE).nil();
        if duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // It cannot be used as material for a Fusion, Synchro, or Link Summon
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, aux::stringid(ID, 3));
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
            effect::set_value_fn(d, e1, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_LINK]));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, token, e1, false);
        }
    }
    duel::special_summon_complete(d);
}
