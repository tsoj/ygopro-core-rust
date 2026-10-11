//! Abyss-strom
use super::prelude::*;

pub const ID: u32 = 97697447;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 face-up "Umi" you control to the GY; send all Spells/Traps on the field to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_SSET));
    card::register_effect(d, c, e1, false);
}
fn spell_trap_grave(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)
}
fn costfilter(d: &mut Duel, c: CardId, hc: CardId) -> bool {
    if !(card::is_code(d, c, &[CARD_UMI]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    let mut ex = CardSet::from_card(c);
    group::add_card(&mut ex, &CardSet::from_card(hc));
    duel::is_existing_matching_card(d, Some(&spell_trap_grave), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| costfilter(d, x, c)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| costfilter(d, x, c)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let exc = if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) { Some(h) } else { None };
        return duel::is_existing_matching_card(d, Some(&spell_trap_grave), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, exc.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let exc = if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    let g = duel::get_matching_group(d, Some(&spell_trap_grave), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc.into());
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
