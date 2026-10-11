//! Nordic Relic Gungnir
use super::prelude::*;

pub const ID: u32 = 15286412;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NORDIC, SET_AESIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NORDIC as u16, SET_AESIR as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST + REASON_TEMPORARY, None, None);
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, Value::card(first));
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_location(d, chkc, LOCATION_ONFIELD) && chkc != h;
    }
    if a.chk == Some(0) {
        effect::set_label(d, a.e, &[0]);
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into()).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && effect::get_label(d, a.e) == 1 {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(retcon)));
    effect::set_operation(d, e1, Some(operation(retop)));
    effect::set_label(d, e1, &[2]);
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, e1, lo);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    duel::register_effect(d, e1, a.tp);
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let ct = effect::get_label(d, a.e) - 1;
    effect::set_label(d, a.e, &[ct]);
    if ct == 0 {
        let tc = effect::get_label_object(d, a.e).as_card().nil();
        duel::return_to_field(d, tc, Some(POS_FACEUP_ATTACK as u8), None);
    }
}
