//! Prohibition
use super::prelude::*;

pub const ID: u32 = 43711255;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // forbidden
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e2, EFFECT_FORBIDDEN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0x7f, 0x7f);
    effect::set_target(d, e2, Some(effect_card_fn(bantg)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
}
fn bantg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    let code = effect::get_label(d, e1);
    card::is_original_code_rule(d, c, &[code as u32])
        && (!card::is_on_field(d, c) || card::get_real_field_id(d, c) > effect::get_field_id(d, e))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    effect::set_label(d, a.e, &[0]);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CODE as u64);
    let ac = duel::announce_card(d, a.tp, &[]);
    duel::set_target_param(d, ac as u32);
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    effect::set_label(d, a.e, &[ac]);
    let h = effect::get_handler(d, a.e).nil();
    card::set_hint(d, h, CHINT_CARD as u8, ac as u64);
    duel::adjust_instantly(d, Some(h));
}
