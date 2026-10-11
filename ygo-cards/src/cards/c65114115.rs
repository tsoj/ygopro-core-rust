//! Seven Stars Slash - Army Annihilator Hagun
use super::prelude::*;

pub const ID: u32 = 65114115;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip 1 Level 7 monster with the same Type, but a different name, to 1 face-up monster you control, from your hand or Deck as an Equip Spell
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // Add this card to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn e1_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && {
        let rac = card::get_race(d, c);
        let code = card::get_code(d, c)[0];
        duel::is_existing_matching_card(d, Some(&|d, x| eqpfilter(d, x, tp, rac, code)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
    }
}
fn eqpfilter(d: &mut Duel, c: CardId, tp: u8, rac: u64, code: u32) -> bool {
    card::is_level(d, c, &[7])
        && card::is_race(d, c, rac)
        && !card::is_code(d, c, &[code])
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp);
    }
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_location(d, c, LOCATION_HAND) {
        ft -= 1;
    }
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_SZONE as i32);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let Some(tc) = duel::first_target(d) else { return };
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let rac = card::get_race(d, tc);
    let code = card::get_code(d, tc)[0];
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| eqpfilter(d, x, tp, rac, code)),
        tp,
        LOCATION_HAND | LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if let Some(ec) = group::get_first(d, &sel) {
        if duel::equip(d, tp, ec, tc, true, false) {
            // The equipped monster gains 700 ATK
            let e1 = effect::create_effect(d, ec);
            effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, 700);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, ec, e1, false);
            // Equip limit
            let e2 = effect::create_effect(d, ec);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
            effect::set_value_fn(d, e2, effect_card_fn(move |_d, _e, c| c == tc));
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, ec, e2, false);
            // Return that Equip Card to the hand during the End Phase
            aux::delayed_operation(
                d,
                ec,
                PHASE_END,
                ID,
                e,
                tp,
                Some(std::sync::Arc::new(move |d: &mut Duel, _g: &CardSet, _a: &EffArgs| {
                    duel::send_to_hand(d, ec, None, REASON_EFFECT, None);
                    true
                })),
                None,
                Some(0),
                Some(1),
                Some(aux::stringid(ID, 2)),
                None,
            );
        }
    }
}
fn thconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_controler(d, c, 1 - tp)
        && (card::get_previous_level_on_field(d, c) >= 8 || card::get_previous_rank_on_field(d, c) >= 8)
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg();
    group::is_exists(d, &eg, &|d, c| thconfilter(d, c, a.tp), 1, NO_EX)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
