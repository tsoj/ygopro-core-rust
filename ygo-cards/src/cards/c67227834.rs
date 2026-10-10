//! Magic Formula
use super::prelude::*;

pub const ID: u32 = 67227834;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_code(d, x, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL]))), None, None, None, None, None, None);
    // Atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 700);
    card::register_effect(d, c, e2, false);
    // recover
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_RECOVER);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_condition(d, e4, Some(condition(reccon)));
    effect::set_target(d, e4, Some(cost_target(rectg)));
    effect::set_operation(d, e4, Some(operation(recop)));
    card::register_effect(d, c, e4, false);
}
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1000);
    true
}
fn recop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
