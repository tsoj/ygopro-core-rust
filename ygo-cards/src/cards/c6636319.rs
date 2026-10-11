//! Evil★Twin Ki-sikil Deal
use super::prelude::*;

pub const ID: u32 = 6636319;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_KI_SIKIL, SET_LIL_LA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 "Ki-Sikil" monster
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_KI_SIKIL as u16], sc, st, Some(tp)))), 1, Some(1), None, None, None);
    // You can only Link Summon "Evil★Twin Ki-sikil Deal" once per turn
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e0, Some(condition(regcond)));
    effect::set_operation(d, e0, Some(operation(regop)));
    card::register_effect(d, c, e0, false);
    // Apply this effect for the rest of this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(effcond)));
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn regcond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    // Prevent another Link Summon of "Evil★Twin Ki-sikil Deal" that turn
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(regtarget)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
/// `function(e,c,sump,sumtype) return c:IsCode(id) and sumtype&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK end`
fn regtarget(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtype = a.int(3) as u32;
    card::is_code(d, c, &[ID]) && (sumtype & SUMMON_TYPE_LINK) == SUMMON_TYPE_LINK
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LIL_LA as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn effcond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::has_flag_effect(d, a.tp, ID, None)
}
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    let c = effect::get_handler(d, a.e).nil();
    // Draw 1 card immediately after it resolves
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_condition(d, e1, Some(condition(drawcon)));
    effect::set_operation(d, e1, Some(operation(drawop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
fn drawcon(d: &mut Duel, a: &EffArgs) -> bool {
    let ch = duel::get_current_chain(d, true) - 1;
    ch > 0
        && a.ep == 1 - a.tp
        && matches!(duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_PLAYER]).first(), Some(Value::Int(p)) if *p == a.tp as i64)
        && chain::is_triggering(d, ch as u8, chain::Prop::Type, &[TYPE_MONSTER as i64]).truthy()
        && chain::is_triggering(d, ch as u8, chain::Prop::SetCard, &[SET_KI_SIKIL as i64, SET_LIL_LA as i64]).truthy()
}
fn drawop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
