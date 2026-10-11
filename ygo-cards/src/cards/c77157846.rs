//! Shade the Obscure
use super::prelude::*;

pub const ID: u32 = 77157846;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    pendulum::add_procedure(d, c, None, None);
    // You cannot Pendulum Summon, except Pendulum Monsters (this effect cannot be negated)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(e1_target)));
    card::register_effect(d, c, e1, false);
    // If a monster(s) with 1000 ATK or less is Special Summoned (except during the Damage Step): You can return this card to the hand. You can only use this effect of "Shade the Obscure" once per turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    effect::set_operation(d, e2, Some(operation(rthop)));
    card::register_effect(d, c, e2, false);
    // If you have a card in your Pendulum Zone and this card is in your hand: You can Special Summon this card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_HAND);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
    // You can pay 1000 LP; destroy 1 Pendulum Monster Card in your hand or face-up field, then you can add 1 Pendulum Monster from your Deck to your face-up Extra Deck
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_TOEXTRA);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
/// `function(e,c,tp,sumtype,sumpos)`
fn e1_target(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtype = a.int(3) as u64;
    !card::is_pendulum_monster(d, c) && (sumtype & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg();
    group::is_exists(d, &eg, &aux::faceup_filter(|d, c| card::is_attack_below(d, c, 1000)), 1, NO_EX)
}
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_PZONE, 0) > 0
}
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn rthop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_original_type(d, c, TYPE_PENDULUM) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, tp, Some(&desfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    if let Some(first) = group::get_first(d, &g) {
        if card::is_on_field(d, first) {
            duel::hint_selection(d, &g, true);
        }
    }
    if duel::destroy(d, &g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_pendulum_monster(d, c) && card::is_able_to_extra(d, c)), tp, LOCATION_DECK, 0, NO_EX);
    if dg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
    let sg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() {
        duel::break_effect(d);
        duel::send_to_extra_p(d, &sg, Some(tp), REASON_EFFECT, None);
    }
}
