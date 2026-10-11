//! Trito Enneacraft - "exapatisIA"
use super::prelude::*;

pub const ID: u32 = 44716748;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_EC as u32], listed_series: &[SET_ENNEACRAFT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_EC as u16, Some(LOCATION_PZONE), None);
    pendulum::add_procedure(d, c, None, None);
    // Each time a monster(s) is flipped face-up, place 1 E.C. Counter on this card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_FLIP);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_operation(d, e1, Some(operation(counterop)));
    card::register_effect(d, c, e1, false);
    // Reveal 3 "Enneacraft" cards from your Deck, your opponent randomly picks 1 for you to add to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(900))));
    effect::set_target(d, e2, Some(cost_target(rvtg)));
    effect::set_operation(d, e2, Some(operation(rvop)));
    card::register_effect(d, c, e2, false);
    // FLIP: Destroy 1 Spell/Trap your opponent controls, and if you do, Special Summon 1 monster from your hand in face-down Defense Position
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_FLIP);
    effect::set_target(d, e3, Some(cost_target(fliptg)));
    effect::set_operation(d, e3, Some(operation(flipop)));
    card::register_effect(d, c, e3, false);
    // Add 1 "Enneacraft" card from your Deck to your hand
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e4, EVENT_CHAINING);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(e4_cond)));
    effect::set_cost(d, e4, Some(cost::self_change_position(POS_FACEUP_DEFENSE)));
    effect::set_target(d, e4, Some(cost_target(thtg)));
    effect::set_operation(d, e4, Some(operation(thop)));
    card::register_effect(d, c, e4, false);
    // Special Summon 1 monster from your hand in face-down Defense Position
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 3));
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e5, LOCATION_HAND);
    effect::set_cost(d, e5, Some(cost::and(vec![cost::self_reveal(), cost_target(spcost)])));
    effect::set_target(d, e5, Some(cost_target(sptg)));
    effect::set_operation(d, e5, Some(operation(spop)));
    card::register_effect(d, c, e5, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(|d, a| card::is_position(d, a.c(0), POS_FACEDOWN_DEFENSE)));
}
fn counterop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !duel::is_chain_solving(d) {
        card::add_counter(d, c, COUNTER_EC as u16, 1, false);
    } else {
        // Place 1 E.C. Counter on this card at the end of the Chain Link
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        effect::set_range(d, e1, LOCATION_PZONE);
        effect::set_operation(
            d,
            e1,
            Some(LuaFn::new(move |d, _a| {
                card::add_counter(d, c, COUNTER_EC as u16, 1, false);
                Value::Nil
            })),
        );
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        card::register_effect(d, c, e1, false);
        // Reset "e1" at the end of the Chain Link
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        effect::set_operation(
            d,
            e2,
            Some(LuaFn::new(move |d, _a| {
                effect::reset(d, e1);
                Value::Nil
            })),
        );
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        duel::register_effect(d, e2, a.tp);
    }
}
fn rvfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16]) && card::is_able_to_hand(d, c, None) && !card::is_public(d, c)
}
fn rvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rvfilter), tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn rvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, tp, Some(&rvfilter), tp, LOCATION_DECK, 0, 3, 3, false, NO_EX).nil();
    if g.len() != 3 {
        return;
    }
    duel::confirm_cards(d, 1 - tp, g.clone());
    let sg = group::random_select(d, &g, 1 - tp, 1);
    duel::shuffle_deck(d, tp);
    if !sg.is_empty() {
        duel::disable_shuffle_check(d, true);
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
fn fliptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn flipop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let dg = duel::select_matching_card(d, tp, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    if dg.is_empty() {
        return;
    }
    duel::hint_selection(d, dg.clone(), true);
    if duel::destroy(d, dg, REASON_EFFECT, None, None) > 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = duel::select_matching_card(
            d,
            tp,
            Some(&|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
            tp,
            LOCATION_HAND,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        if !sg.is_empty() {
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        }
    }
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // You cannot Special Summon the turn you activate this effect, except in face-down Defense Position
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 4));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_OATH | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_FORCE_SPSUMMON_POSITION);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value(d, e1, POS_FACEDOWN_DEFENSE as i64);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
                tp,
                LOCATION_HAND,
                0,
                1,
                NO_EX,
            );
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
        tp,
        LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn e4_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if !(card::is_facedown(d, h) && a.ep == 1 - a.tp) {
        return false;
    }
    effect::is_has_category(d, a.re.nil(), CATEGORY_SET)
}
