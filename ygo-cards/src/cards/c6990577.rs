//! Guardragon Andrake
use super::prelude::*;

pub const ID: u32 = 6990577;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_unsummonable(d, c);
    card::add_must_be_special_summoned_by_card_effect(d, c);
    // Double this card's original ATK/DEF, until the end of the next turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(lua_fn(cond1)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(lua_fn(cond2)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_DECK) end`
fn cond1(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_previous_location(d, c, LOCATION_HAND | LOCATION_DECK)
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_GRAVE|LOCATION_REMOVED) end`
fn cond2(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_previous_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let atk = card::get_base_attack(d, c);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, a.tp, atk);
    let def = card::get_base_defense(d, c);
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(CardSet::from_card(c)), 1, a.tp, def);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        let v = card::get_base_attack(d, c) * 2;
        effect::set_value(d, e1, v as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        card::register_effect(d, c, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
        let v2 = card::get_base_defense(d, c) * 2;
        effect::set_value(d, e2, v2 as i64);
        card::register_effect(d, c, e2, false);
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
