//! Uni-Horned Familiar
use super::prelude::*;

pub const ID: u32 = 22318971;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_position(d, h, POS_FACEUP_DEFENSE)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, 0, 1, 1, false, h.into()).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, h, None, None, None);
    }
    let attacker = duel::get_attacker(d).nil();
    duel::set_target_card(d, attacker);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) && duel::remove(d, c, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_condition(d, e1, Some(condition(retcon)));
        effect::set_operation(d, e1, Some(operation(retop)));
        duel::register_effect(d, e1, tp);
        let ac = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, ac, e) && card::is_faceup(d, ac) {
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_MUST_ATTACK);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, ac, e2, false);
        }
    }
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let owner = a.e.get_owner(d);
    duel::return_to_field(d, owner, None, None);
}
