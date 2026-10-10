//! Ojama Duo
use super::prelude::*;

pub const ID: u32 = 14470845;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_OJAMA], listed_names: &[TOKEN_OJAMA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 2 tokens to opponent's field
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Special summon 2 "Ojama" monsters from deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_condition(d, e2, Some(aux::exccon()));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e2, false);
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_OJAMA as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(1000),
        level: Some(2),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 1
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_OJAMA, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 < 2
        || !duel::is_player_can_special_summon_monster(d, tp, TOKEN_OJAMA, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), 0)
    {
        return;
    }
    for i in 0..2 {
        let token = duel::create_token(d, tp, TOKEN_OJAMA_DUO + i).nil();
        if duel::special_summon_step(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // Cannot be tributed for a tribute summon
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3304);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_value(d, e1, 1);
            card::register_effect(d, token, e1, true);
            // Inflict 300 damage when destroyed
            let h = effect::get_handler(d, a.e).nil();
            let e2 = effect::create_effect(d, h);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e2, EVENT_LEAVE_FIELD);
            effect::set_operation(d, e2, Some(operation(damop)));
            card::register_effect(d, token, e2, true);
        }
    }
    duel::special_summon_complete(d);
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_reason(d, c, REASON_DESTROY) {
        let pc = card::get_previous_controler(d, c);
        duel::damage(d, pc, 300, REASON_EFFECT, false, None);
    }
    effect::reset(d, a.e);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_OJAMA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none() && ft > 1 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 1 {
        if let Ok(sg) = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false) {
            duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
            duel::confirm_cards(d, 1 - tp, sg);
        }
    }
}
