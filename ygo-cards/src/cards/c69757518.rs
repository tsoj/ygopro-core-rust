//! Number C5: Chaos Chimera Dragon
use super::prelude::*;

pub const ID: u32 = 69757518;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(5), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3+ Level 6 monsters
    xyz::add_procedure(d, c, None, Some(6), 3, None, None, Some(xyz::MaxCt::Infinite), None, false, None);
    // Gains 1000 ATK for each material attached to it
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // This card can attack an opponent's monster again in a row
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e2, Some(condition(atcon)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_operation(d, e2, Some(operation(atop)));
    card::register_effect(d, c, e2, false);
    // Place 1 card from the GY on the top of the Deck, and if you do, attach another to this card as material
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TODECK);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(cost)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_overlay_count(d, c) as i64 * 1000
}
/// `s.atcon(e,tp,...)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(c) && card::can_chain_attack(d, c, Some(0), true)
}
/// `s.atop(e,tp,...)`
fn atop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_battle(d, c) {
        return;
    }
    duel::chain_attack(d, None);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE | PHASE_DAMAGE_CAL, 1);
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_type(d, h, TYPE_XYZ)
            && duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 2, 2, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 2, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let mut sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    if sg.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let sg1 = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
        let c = effect::get_handler(d, a.e).nil();
        if duel::send_to_deck(d, sg1.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0 && card::is_relate_to_effect(d, c, a.e) {
            group::remove_card(&mut sg, &sg1);
            duel::overlay(d, c, sg, false);
        }
    }
}
