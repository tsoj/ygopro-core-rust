//! Lightsworn Barrier
use super::prelude::*;

pub const ID: u32 = 22201234;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // quick
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    effect::set_condition(d, e2, Some(condition(qcon)));
    effect::set_cost(d, e2, Some(cost_target(qcost)));
    effect::set_target(d, e2, Some(cost_target(qtg)));
    effect::set_operation(d, e2, Some(operation(qop)));
    card::register_effect(d, c, e2, false);
}
fn qcon(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attack_target(d).nil();
    card::is_faceup(d, at) && card::is_set_card(d, at, &[SET_LIGHTSWORN as u16]) && card::is_controler(d, at, a.tp)
}
fn qcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, a.tp, 2) && card::get_flag_effect(d, h, ID) == 0;
    }
    card::register_flag_effect(d, h, ID, RESET_CHAIN, 0, 1, 0, 0);
    duel::discard_deck(d, a.tp, 2, REASON_COST);
    true
}
fn qtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tg = duel::get_attacker(d);
    if let Some(chkc) = a.chkc() {
        return tg == Some(chkc);
    }
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e));
    }
    duel::set_target_card(d, tg.nil());
    true
}
fn qop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    duel::negate_attack(d);
}
