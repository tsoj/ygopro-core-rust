//! Oasis of Dragon Souls
use super::prelude::*;

pub const ID: u32 = 39122311;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e2, Some(operation(checkop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e3, Some(operation(desop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
    // destroy2
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e4, Some(condition(descon2)));
    effect::set_operation(d, e4, Some(operation(desop2)));
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && duel::special_summon_step(d, tc.nil(), 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None)
    {
        card::set_card_target(d, c, tc.nil());
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e1, RACE_WYRM as i64);
        effect::set_condition(d, e1, Some(condition(rcon)));
        card::register_effect(d, tc.nil(), e1, false);
    }
    duel::special_summon_complete(d);
}
/// `s.rcon(e)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetOwner():IsHasCardTarget(e:GetHandler())
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h)
}
/// `s.checkop(e,...)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, h) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let le = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, le) != 0 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, h) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.descon2(e,tp,eg,...)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    match card::get_first_card_target(d, h) {
        Some(tc) => a.eg.clone().nil().as_slice().contains(&tc),
        None => false,
    }
}
/// `s.desop2(e,...)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
