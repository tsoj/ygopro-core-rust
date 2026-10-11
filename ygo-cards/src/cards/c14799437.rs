//! Infernoid Onuncu
use super::prelude::*;

pub const ID: u32 = 14799437;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    aux::infernoid::register_summon_procedure(d, c, 3);
    // Destroy all monsters on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Negate the activation of a Spell/Trap card or effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, h.into());
    }
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && effect::is_spell_trap_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let rc = effect::get_handler(d, a.re.nil()).nil();
    let relation = card::is_relate_to_effect(d, rc, a.re.nil());
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(tp), None, None) || (!relation && duel::is_player_can_remove(d, tp, None));
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    if relation {
        let ctl = card::get_controler(d, rc);
        let loc = card::get_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, ctl, loc as i32);
    } else {
        let pl = card::get_previous_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, pl as i32);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            duel::remove(d, a.eg.clone().nil(), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
