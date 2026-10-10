//! Dark Magician of Chaos
use super::prelude::*;

pub const ID: u32 = 40737112;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // remove
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
    // redirect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_LEAVE_FIELD_REDIRECT);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_condition(d, e3, Some(condition(recon)));
    effect::set_value(d, e3, LOCATION_REMOVED as i64);
    card::register_effect(d, c, e3, false);
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        effect::set_label(d, ge1, &[ID as i64]);
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_operation(d, ge1, Some(aux::sumreg()));
        duel::register_effect(d, ge1, 0);
        let ge2 = effect::clone(d, ge1);
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        effect::set_label(d, ge2, &[ID as i64]);
        duel::register_effect(d, ge2, 0);
    });
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::get_flag_effect(d, c, ID) != 0;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, c, ID);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    effect::set_label_object(d, a.e, Value::card(bc));
    match bc {
        Some(bc) => card::is_status(d, bc, STATUS_BATTLE_DESTROYED) && card::is_status(d, c, STATUS_OPPO_BATTLE),
        None => false,
    }
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let lo = effect::get_label_object(d, a.e).as_card().map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, lo, 1, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_battle(d, bc) && card::is_able_to_remove(d, bc, None, None, None) {
        duel::remove(d, bc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
fn recon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_faceup(d, c)
}
