//! Transonic Bird
use super::prelude::*;

pub const ID: u32 = 34072799;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[17888577], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Add 1 Ritual monster from the Deck to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Perform a Ritual summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    effect::set_cost(d, e2, Some(cost_target(ritcost)));
    effect::set_target(d, e2, Some(cost_target(rittg)));
    effect::set_operation(d, e2, Some(operation(ritop)));
    card::register_effect(d, c, e2, false);
    // Negate the effects of 1 face-up card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_DISABLE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_RELEASE);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_ritual_summoned(d, h)
    })));
    effect::set_target(d, e3, Some(cost_target(negtg)));
    effect::set_operation(d, e3, Some(operation(negop)));
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter1(c,tp)`
fn thfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_ritual_spell(d, c) && !card::is_public(d, c) && card::is_able_to_deck(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| thfilter2(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter2(c,mc)`
fn thfilter2(d: &mut Duel, c: CardId, mc: CardId) -> bool {
    card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None) && isfit(d, c, mc)
}
/// `s.isfit(c,mc)`
fn isfit(d: &mut Duel, c: CardId, mc: CardId) -> bool {
    let fit = crate::class::fit_monster(d, mc);
    (!fit.is_empty() && card::is_code(d, c, &fit)) || {
        let codes = card::get_code(d, c);
        card::lists_code(d, mc, &codes)
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter1(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter1(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let Some(rc) = group::get_first(d, &g) else { return };
    duel::confirm_cards(d, 1 - tp, rc);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g2 = duel::select_matching_card(d, tp, Some(&|d, c| thfilter2(d, c, rc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g2) {
        if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0 {
            duel::confirm_cards(d, 1 - tp, tc);
            duel::send_to_deck(d, rc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.ritcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ritcost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.ritfilter(c)`
fn ritfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_ritual_spell(d, c) && card::is_able_to_grave_as_cost(d, c) && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// `s.rittg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rittg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) == 0 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return duel::is_existing_matching_card(d, Some(&ritfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    effect::set_label(d, a.e, &[0]);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&ritfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, tc, true, true, false).map(|x| x.0);
    effect::set_label_object(d, a.e, Value::effect(te));
    duel::send_to_grave(d, tc, REASON_COST, None, None);
    let te = te.nil();
    let prop = effect::get_property(d, te);
    effect::set_property(d, a.e, prop.0, prop.1);
    let tg = effect::get_target(d, te);
    if let Some(tg) = tg {
        let args = [
            Value::Effect(a.e),
            Value::Int(tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(1),
        ];
        tg.call(d, &args);
    }
    duel::clear_operation_info(d, 0);
    true
}
/// `s.ritop(e,tp,eg,ep,ev,re,r,rp)`
fn ritop(d: &mut Duel, a: &EffArgs) {
    let te = match effect::get_label_object(d, a.e) {
        Value::Effect(te) => te,
        _ => return,
    };
    if let Some(op) = effect::get_operation(d, te) {
        let args = [
            Value::Effect(a.e),
            Value::Int(a.tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
        ];
        op.call(d, &args);
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, tp, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if ((card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER)) && card::is_relate_to_effect(d, tc, a.e) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e3, false);
        }
    }
}
