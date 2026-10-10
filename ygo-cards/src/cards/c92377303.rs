//! Dark Sage
use super::prelude::*;

pub const ID: u32 = 92377303;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EVENT_CUSTOM + 71625222);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_range(d, e2, LOCATION_DECK);
    card::register_effect(d, c, e2, false);
    // Add 1 Spell to hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e3, Some(condition(thcon)));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp
}
fn costfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN])
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, ft, tp)), 1, 1, false, None, NO_EX);
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_DECK) {
        duel::confirm_cards(d, 1 - tp, &CardSet::from_card(h));
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, ft, tp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, true, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, card::get_location(d, h) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None) != 0 {
        card::complete_procedure(d, c);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    effect::get_handler(d, a.e).nil() == effect::get_handler(d, a.re.nil()).nil()
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
