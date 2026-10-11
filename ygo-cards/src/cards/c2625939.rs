//! Spool Code
use super::prelude::*;

pub const ID: u32 = 2625939;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[2625940], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp)
        && duel::get_attack_target(d).is_none()
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_CYBERSE)), a.tp, LOCATION_GRAVE, 0, 3, NO_EX)
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_LIGHT,POS_FACEUP_DEFENSE)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_CYBERSE),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    if !duel::negate_attack(d) {
        return;
    }
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft == 0
        || !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), Some(POS_FACEUP_DEFENSE as u8), None, 0)
        || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        return;
    }
    ft = ft.min(3);
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    if ft > 1 {
        ft = duel::announce_number_range(d, a.tp, Some(1), Some(ft as u32), &[]).nil().0 as i32;
    }
    duel::break_effect(d);
    for _ in 0..ft {
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
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
    }
    duel::special_summon_complete(d);
}
