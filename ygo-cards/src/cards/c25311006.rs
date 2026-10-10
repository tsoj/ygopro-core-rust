//! Triple Tactics Talent
use super::prelude::*;

pub const ID: u32 = 25311006;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(counter_filter));
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_custom_activity_count(d, ID, 1 - a.tp, ACTIVITY_CHAIN) > 0
}
// function(re,tp,cid) return not (Duel.IsMainPhase() and re:IsMonsterEffect()) end
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    !(duel::is_main_phase(d, None) && effect::is_monster_effect(d, a.e(0)))
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let b1 = duel::is_player_can_draw(d, a.tp, 2);
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    let b3 = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    if op == Some(1) {
        effect::set_category(d, a.e, CATEGORY_DRAW);
        effect::set_property(d, a.e, EFFECT_FLAG_PLAYER_TARGET, 0);
        duel::set_target_player(d, a.tp);
        duel::set_target_param(d, 2);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    } else if op == Some(2) {
        effect::set_category(d, a.e, CATEGORY_CONTROL);
        effect::set_property(d, a.e, 0, 0);
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, a.tp, 0);
    } else if op == Some(3) {
        effect::set_category(d, a.e, CATEGORY_TODECK);
        effect::set_property(d, a.e, EFFECT_FLAG_PLAYER_TARGET, 0);
        duel::set_target_player(d, a.tp);
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, 1 - a.tp, LOCATION_HAND as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    if op == 1 {
        //Draw 2 cards
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    } else if op == 2 {
        //Take control of 1 monster your opponent controls until the End Phase
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::get_control(d, g, a.tp, PHASE_END as u16, 1, None, None);
        }
    } else if op == 3 {
        //Look at your opponent's hand, and choose 1 card from it to shuffle into the Deck
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        let g = duel::get_field_group(d, p, 0, LOCATION_HAND);
        if !g.is_empty() {
            duel::confirm_cards(d, p, g.clone());
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
            let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
            if !sg.is_empty() {
                duel::hint_selection(d, sg.clone(), true);
                duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            }
            duel::shuffle_hand(d, 1 - p);
        }
    }
}
