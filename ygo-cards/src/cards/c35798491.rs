//! Darkbishop Archfiend
use super::prelude::*;

pub const ID: u32 = 35798491;

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
    // Roll a six-sided die when resolving an opponent's card effect that targets an "Archfiend" monster you control
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    if duel::check_lp_cost(d, a.tp, 500) {
        duel::pay_lp_cost(d, a.tp, 500);
    } else {
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
fn filter(d: &mut Duel, c: CardId, tp: u8, re: EffectId) -> bool {
    card::is_set_card(d, c, &[SET_ARCHFIEND as u16])
        && card::is_relate_to_effect(d, c, re)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_faceup(d, c)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    if !(effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) && a.ep == 1 - a.tp) {
        return;
    }
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    let Some(tg) = tg else { return };
    if !group::is_exists(d, &tg, &|d, c| filter(d, c, a.tp, re), 1, NO_EX) || !duel::is_chain_disablable(d, a.ev as u8) {
        return;
    }
    let rc = effect::get_handler(d, re);
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    if dc == 1 || dc == 3 || dc == 6 {
        if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc.nil(), re) {
            duel::destroy(d, rc.nil(), REASON_EFFECT, None, None);
        }
    }
}
