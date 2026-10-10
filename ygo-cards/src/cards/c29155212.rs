//! Pumpking the King of Ghosts
use super::prelude::*;

pub const ID: u32 = 29155212;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[62121], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(adval));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
/// `aux.FaceupFilter(Card.IsCode,62121)`
fn pumpking_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[62121])
}
/// `s.adval(e,c)`
fn adval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let flag = card::get_flag_effect(d, c, ID) as i64;
    if duel::is_existing_matching_card(d, Some(&pumpking_filter), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX) {
        100 + flag * 100
    } else {
        flag * 100
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_turn_player(d, a.tp) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) > 4 {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&pumpking_filter), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
}
