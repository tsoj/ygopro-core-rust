//! S-Force Signify
use super::prelude::*;

pub const ID: u32 = 19951423;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[19951423], listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Normal or Special Summoned: You can add 1 "S-Force" monster from your Deck to your hand, except "S-Force Signus", then if an opponent's monster is in this card's column, you can draw 1 card
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_DRAW);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_target(d, e1a, Some(cost_target(thtg)));
    effect::set_operation(d, e1a, Some(operation(thop)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
    // If an opponent's monster activates its effect in the same column as your monster: You can make all "S-Force" monsters you currently control gain 1000 ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_S_FORCE as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 0);
    true
}
fn columnfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, opp)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() || duel::send_to_hand(d, &g, None, REASON_EFFECT, None) == 0 {
        return;
    }
    let first = group::get_first(d, &g).nil();
    if !card::is_location(d, first, LOCATION_HAND) {
        return;
    }
    duel::confirm_cards(d, 1 - tp, &g);
    duel::shuffle_hand(d, tp);
    duel::shuffle_deck(d, tp);
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::is_player_can_draw(d, tp, 1) {
        let cg = card::get_column_group(d, c, 0, 0);
        if group::is_exists(d, &cg, &|d, x| columnfilter(d, x, 1 - tp), 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::break_effect(d);
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SEQUENCE]);
    let trig_ctrl_ok = matches!(info[0], Value::Int(x) if x == (1 - tp) as i64);
    if !(trig_ctrl_ok && matches!(info[1], Value::Int(x) if x == LOCATION_MZONE as i64)) {
        return false;
    }
    let seq = info[2].int() as u32;
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_column(d, c, seq, Some(1 - tp), Some(LOCATION_MZONE))),
        tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_S_FORCE as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_S_FORCE as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    for tc in group::iter(d, &g) {
        // All "S-Force" monsters you currently control gain 1000 ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
