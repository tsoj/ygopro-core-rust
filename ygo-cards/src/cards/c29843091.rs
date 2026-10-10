//! Ojama Trio
use super::prelude::*;

pub const ID: u32 = 29843091;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_OJAMA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 3 tokens to opponent's field
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
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
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>2
        //   and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_OJAMA,SET_OJAMA,TYPES_TOKEN,0,1000,2,RACE_BEAST,ATTRIBUTE_LIGHT,POS_FACEUP_DEFENSE,1-tp)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 2
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_OJAMA, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 3, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 < 3
        || !duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_OJAMA, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), 0)
    {
        return;
    }
    for i in 0..3 {
        let token = duel::create_token(d, a.tp, TOKEN_OJAMA + i).nil();
        if duel::special_summon_step(d, token, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
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
        let p = card::get_previous_controler(d, c);
        duel::damage(d, p, 300, REASON_EFFECT, false, None);
    }
    effect::reset(d, a.e);
}
