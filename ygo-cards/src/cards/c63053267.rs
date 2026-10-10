//! Nagel's Protection
use super::prelude::*;

pub const ID: u32 = 63053267;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TINDANGLE], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // "Tindangle" monsters in your Main Monster Zones cannot be destroyed by battle or your opponent's card effects
    let e1a = effect::create_effect(d, c);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1a, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e1a, LOCATION_SZONE);
    effect::set_target_range(d, e1a, LOCATION_MMZONE, 0);
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TINDANGLE as u16]))));
    effect::set_value(d, e1a, 1);
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e1b, aux::indoval());
    card::register_effect(d, c, e1b, false);
    // Once per turn, if your "Tindangle" monster inflicts battle damage to your opponent, the damage is doubled
    let e2a = effect::create_effect(d, c);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2a, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2a, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_range(d, e2a, LOCATION_SZONE);
    effect::set_target_range(d, e2a, LOCATION_MZONE, 0);
    effect::set_condition(d, e2a, Some(condition(e2a_cond)));
    effect::set_target(d, e2a, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TINDANGLE as u16]))));
    effect::set_value_fn(d, e2a, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    card::register_effect(d, c, e2a, false);
    // Register if your "Tindangle" monster inflicts battle damage to your opponent
    let e2b = effect::create_effect(d, c);
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2b, EVENT_PRE_BATTLE_DAMAGE);
    effect::set_range(d, e2b, LOCATION_SZONE);
    effect::set_condition(d, e2b, Some(condition(e2b_cond)));
    effect::set_operation(d, e2b, Some(operation(e2b_op)));
    card::register_effect(d, c, e2b, false);
    // Add 1 "Nagel's Protection" from your Deck to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(
        d,
        e3,
        Some(cost::and(vec![
            cost::self_banish(),
            cost::discard(Some(std::sync::Arc::new(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_TINDANGLE as u16]))), false, None, None, None),
        ])),
    );
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn e2a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::has_flag_effect(d, h, ID, None)
}
fn e2b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp && {
        let eg = a.eg.clone().nil();
        let first = group::get_first(d, &eg).nil();
        card::is_set_card(d, first, &[SET_TINDANGLE as u16])
    }
}
fn e2b_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0 {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
