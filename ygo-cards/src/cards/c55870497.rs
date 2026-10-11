//! Burning Bamboo Sword
use super::prelude::*;

pub const ID: u32 = 55870497;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BAMBOO_SWORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // set
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(skipcon)));
    effect::set_operation(d, e3, Some(operation(skipop)));
    card::register_effect(d, c, e3, false);
}
// function s.skipcon(e,tp,eg,ep,ev,re,r,rp)
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    a.rp == a.tp && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
        let rc = effect::get_handler(d, re).nil();
        card::is_set_card(d, rc, &[SET_BAMBOO_SWORD as u16])
    }
}
// function s.skipop(e,tp,eg,ep,ev,re,r,rp)
fn skipop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SKIP_M1);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    duel::register_effect(d, e1, a.tp);
}
