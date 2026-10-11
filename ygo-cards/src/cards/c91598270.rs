//! SolSolfachord Gracia
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 91598270;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum
    pendulum::add_procedure(d, c, None, None);
    // cannot activate s/t on pendulum summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(sucop)));
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // cannot activate monster effects on attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(btcon)));
    effect::set_operation(d, e4, Some(operation(btop)));
    card::register_effect(d, c, e4, false);
}
fn sucfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_controler(d, c, tp) && card::is_pendulum_summoned(d, c)
}
fn sucop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| sucfilter(d, c, tp), 1, NO_EX) {
        duel::set_chain_limit_till_chain_end(d, lua_fn(|d, a| chainlm(d, a)));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let rp = a.p(1);
    let tp = a.p(2);
    tp == rp || (effect::is_spell_trap_effect(d, e) && !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE))
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn btcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    match duel::get_attacker(d) {
        None => false,
        Some(ac) => card::is_controler(d, ac, tp) && card::is_type(d, ac, TYPE_PENDULUM) && card::is_set_card(d, ac, &[SET_SOLFACHORD as u16]),
    }
}
fn btop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_condition(d, e1, Some(lua_fn(|d, a| actcon(d, a))));
    effect::set_value_fn(d, e1, lua_fn(|d, a| actlimit(d, a)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    card::register_effect(d, c, e1, false);
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_even_scale(d, c))), p, LOCATION_PZONE, 0, 1, NO_EX)
}
/// `s.actlimit(e,re,tp)`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_monster_effect(d, re)
}
