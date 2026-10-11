//! Harpie Perfumer
use super::prelude::*;

pub const ID: u32 = 39392286;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HARPIE], listed_names: &[CARD_HARPIE_LADY, CARD_HARPIE_LADY_SISTERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // change name
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e3, EFFECT_CHANGE_CODE);
    effect::set_range(d, e3, LOCATION_MZONE | LOCATION_GRAVE);
    effect::set_value(d, e3, CARD_HARPIE_LADY as i64);
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::lists_code(d, c, &[CARD_HARPIE_LADY_SISTERS]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HARPIE as u16]) && card::is_level_above(d, c, 5)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let mut sg1 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // g:Remove(Card.IsCode,nil,sg1:GetFirst():GetCode())
    let first = group::get_first(d, &sg1).nil();
    let codes = card::get_code(d, first);
    group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes), NO_EX);
    if !g.is_empty()
        && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, 210)
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let sg2 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::add_card(&mut sg1, &sg2);
    }
    duel::send_to_hand(d, &sg1, None, REASON_EFFECT, None);
    duel::confirm_cards(d, 1 - tp, &sg1);
}
