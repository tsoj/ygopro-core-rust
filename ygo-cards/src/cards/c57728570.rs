//! Crush Card Virus
use super::prelude::*;

pub const ID: u32 = 57728570;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsAttackBelow(1000)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_attack_below(d, c, 1000)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_above(d, c, 1500)
}
fn hgfilter(d: &mut Duel, c: CardId) -> bool {
    !card::is_public(d, c) || filter(d, c)
}
fn fgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) || filter(d, c)
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_location(d, c, LOCATION_HAND) && card::is_public(d, c)) || (card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)))
        && filter(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&hgfilter), tp, 0, LOCATION_HAND, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&fgfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, 0, LOCATION_MZONE | LOCATION_HAND, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local conf=Duel.GetFieldGroup(tp,0,LOCATION_MZONE|LOCATION_HAND)
    let conf = duel::get_field_group(d, tp, 0, LOCATION_MZONE | LOCATION_HAND);
    let mut ct: i32 = 0;
    if !conf.is_empty() {
        duel::confirm_cards(d, tp, conf.clone());
        // local dg=conf:Filter(s.filter,nil)
        let dg = group::filter(d, &conf, &filter, NO_EX);
        ct = duel::destroy(d, dg, REASON_EFFECT, None, None);
        duel::shuffle_hand(d, 1 - tp);
    }
    let g = duel::get_matching_group(d, Some(&filter), 1 - tp, LOCATION_DECK, 0, NO_EX);
    if ct > 0 && !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 0)) {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_DESTROY as u64);
        let dg = group::select(d, &g, 1 - tp, 1, 3, false, NO_EX).nil();
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e1, 0, 1);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        duel::register_effect(d, e1, tp);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
        duel::register_effect(d, e2, tp);
        let e3 = effect::create_effect(d, h);
        effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_description(d, e3, aux::stringid(ID, 1));
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 2);
        effect::set_target_range(d, e3, 0, 1);
        duel::register_effect(d, e3, tp);
    }
}
