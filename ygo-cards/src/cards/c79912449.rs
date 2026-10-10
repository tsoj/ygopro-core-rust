//! Mikanko Reflection Rondo
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 79912449;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIKANKO as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e0 = aux::add_equip_procedure(d, c, Some(1), None, Some(effect_card_fn(eqlimit)), None, None, None, None, None);
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // Can only control 1 "Inviting Rondo of the Mikanko"
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Gain control of the monster while you control a "Mikanko" monster
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_SET_CONTROL);
    effect::set_value_fn(d, e1, lua_fn(control_value));
    effect::set_condition(d, e1, Some(condition(contcond)));
    card::register_effect(d, c, e1, false);
    // Equipped monster cannot activate its effect while under your control
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
    effect::set_condition(d, e2, Some(condition(actcond)));
    card::register_effect(d, c, e2, false);
    // Send the equiped monster to the GY
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e3, Some(operation(tgop)));
    card::register_effect(d, c, e3, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c) || effect::get_handler_player(d, e) != card::get_controler(d, c)
}
/// `function(e) return e:GetHandlerPlayer() end`
fn control_value(d: &mut Duel, a: Args) -> i64 {
    effect::get_handler_player(d, a.e(0)) as i64
}
/// `s.contcond(e)`
fn contcond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_MIKANKO as u16]))), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.actcond(e)`
fn actcond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    card::get_controler(d, ec) == effect::get_handler_player(d, a.e)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if let Some(ec) = card::get_equip_target(d, h) {
        if card::is_location(d, ec, LOCATION_MZONE) {
            duel::send_to_grave(d, ec, REASON_EFFECT, None, None);
        }
    }
}
