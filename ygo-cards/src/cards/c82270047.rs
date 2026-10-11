//! Kozmo Lightsword
use super::prelude::*;

pub const ID: u32 = 82270047;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KOZMO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, cc, _e, _tp| filter(d, cc))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // atk/def up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
    // extra attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_EXTRA_ATTACK_MONSTER);
    effect::set_value(d, e4, 1);
    card::register_effect(d, c, e4, false);
    // pierce
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e6, EFFECT_PIERCE);
    card::register_effect(d, c, e6, false);
    // tohand
    let e8 = effect::create_effect(d, c);
    effect::set_description(d, e8, aux::stringid(ID, 0));
    effect::set_category(d, e8, CATEGORY_TOHAND);
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e8, EVENT_TO_GRAVE);
    effect::set_property(d, e8, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e8, 1, ID, 0, 0);
    effect::set_condition(d, e8, Some(condition(thcon)));
    effect::set_cost(d, e8, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    effect::set_target(d, e8, Some(cost_target(thtg)));
    effect::set_operation(d, e8, Some(operation(thop)));
    card::register_effect(d, c, e8, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_KOZMO as u16]) && card::is_race(d, c, RACE_PSYCHIC)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
