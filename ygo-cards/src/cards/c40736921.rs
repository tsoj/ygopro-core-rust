//! Xing Zhen Hu Replica
use super::prelude::*;

pub const ID: u32 = 40736921;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_condition(d, e2, Some(aux::persistent_tg_con()));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    //trigger
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    card::register_effect(d, c, e3, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_facedown(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    duel::select_target(d, a.tp, Some(&|d, x| card::is_facedown(d, x)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, c.into());
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        duel::set_chain_limit(d, aux_false());
    }
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let targets = info[0].as_group().cloned().nil();
    let tc = group::get_first(d, &targets);
    if card::is_relate_to_effect(d, c, a.re.nil()) {
        if let Some(tc) = tc {
            if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.re.nil()) {
                card::set_card_target(d, c, tc);
            }
        }
    }
}
