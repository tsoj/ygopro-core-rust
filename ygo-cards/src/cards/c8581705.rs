//! Infernalqueen Archfiend
use super::prelude::*;

pub const ID: u32 = 8581705;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Once per turn, during your Standby Phase, you must pay 500 LP (this is not optional), or this card is destroyed
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    effect::set_operation(d, e1, Some(operation(mtop)));
    card::register_effect(d, c, e1, false);
    // Roll a six-sided die when resolving an opponent's card effect that targets this card
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // Increase the ATK of one "Archfiend" monster by 1000
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(atktg)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    if duel::check_lp_cost(d, a.tp, 500) {
        duel::pay_lp_cost(d, a.tp, 500);
    } else {
        let c = effect::get_handler(d, a.e).nil();
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
fn disop(d: &mut Duel, a: &EffArgs) {
    if !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_relate_to_effect(d, c, a.re.nil()) && a.ep == 1 - a.tp) {
        return;
    }
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    let Some(tg) = tg else { return };
    if !group::is_contains(&tg, c) || !duel::is_chain_disablable(d, a.ev as u8) {
        return;
    }
    let rc = effect::get_handler(d, a.re.nil());
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    if dc != 2 && dc != 5 {
        return;
    }
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc.nil(), a.re.nil()) {
        duel::destroy(d, rc.nil(), REASON_EFFECT, None, None);
    }
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARCHFIEND as u16])
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g, 1, a.tp, 1000);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, 1000);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
