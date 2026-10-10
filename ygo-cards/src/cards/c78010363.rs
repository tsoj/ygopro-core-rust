//! Witch of the Black Forest
use super::prelude::*;

pub const ID: u32 = 78010363;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // c:IsDefenseBelow(1500) and c:IsMonster() and c:IsAbleToHand()
    card::is_defense_below(d, c, 1500) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        let tc = group::get_first(d, &g).nil();
        if card::is_location(d, tc, LOCATION_HAND) {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
            effect::set_target_range(d, e1, 1, 0);
            effect::set_value_fn(d, e1, lua_fn(aclimit));
            let code = card::get_code(d, tc)[0];
            effect::set_label(d, e1, &[code as i64]);
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e1, a.tp);
        }
    }
}
fn aclimit(d: &mut Duel, a: Args<'_>) -> bool {
    // return re:GetHandler():IsCode(e:GetLabel())
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    let label = effect::get_label(d, a.e(0));
    card::is_code(d, h, &[label as u32])
}
