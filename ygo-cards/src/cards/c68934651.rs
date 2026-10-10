//! Firewall Dragon Darkfluid
use super::prelude::*;

pub const ID: u32 = 68934651;
const COUNTER_FW: u16 = 0x14c;
const TYPES: u32 = TYPE_FUSION + TYPE_RITUAL + TYPE_SYNCHRO + TYPE_XYZ;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:EnableCounterPermit(COUNTER_FW)
    card::enable_counter_permit(d, c, COUNTER_FW, None, None);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),3)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        3,
        None,
        None,
        None,
        None,
    );
    // counter
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e1, Some(condition(ctcon)));
    effect::set_target(d, e1, Some(cost_target(cttg)));
    effect::set_operation(d, e1, Some(operation(ctop)));
    card::register_effect(d, c, e1, false);
    // atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
    // negate
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_NEGATE);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(discon)));
    effect::set_cost(d, e3, Some(cost_target(discost)));
    effect::set_target(d, e3, Some(cost_target(distg)));
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPES) && card::is_race(d, c, RACE_CYBERSE)
}
/// `getcount(tp)`
fn getcount(d: &mut Duel, tp: u8) -> i64 {
    let mut tottype: u32 = 0;
    let g = duel::get_matching_group(d, Some(&ctfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    for c in group::iter(d, &g) {
        tottype |= card::get_type(d, c);
    }
    tottype &= TYPES;
    let mut ct = 0;
    while tottype != 0 {
        if tottype & 0x1 != 0 {
            ct += 1;
        }
        tottype >>= 1;
    }
    ct
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = getcount(d, a.tp);
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return ct > 0 && card::is_can_add_counter(d, c, COUNTER_FW, ct as u16, false, 0);
    }
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) || card::is_facedown(d, c) {
        return;
    }
    let ct = getcount(d, a.tp);
    card::add_counter(d, c, COUNTER_FW, ct as u16, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_counter(d, c, COUNTER_FW) as i64 * 2500
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp
        && effect::is_monster_effect(d, re)
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, tp, COUNTER_FW, 1, REASON_COST);
    }
    card::remove_counter(d, c, tp, COUNTER_FW, 1, REASON_COST);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    let tc = duel::get_attacker(d);
    if duel::negate_activation(d, a.ev as u8) && Some(c) == tc && card::can_chain_attack(d, c, Some(0), false) && card::is_relate_to_effect(d, c, e) {
        // Needed for the end of the Damage Step
        duel::chain_attack(d, None);
        // Make another attack in a row
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_operation(d, e1, Some(operation(chainop)));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        card::register_effect(d, c, e1, false);
    }
}
/// `s.chainop(e)`
fn chainop(d: &mut Duel, _a: &EffArgs) {
    duel::chain_attack(d, None);
}
