//! Necrovalley Throne
use super::prelude::*;

pub const ID: u32 = 37561138;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAVEKEEPERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GRAVEKEEPERS) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GRAVEKEEPERS) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    let b2 = duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = if b1 && b2 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else if b1 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)])
    } else {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) + 1
    };
    effect::set_label(d, a.e, &[op as i64]);
    if op == 0 {
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        effect::set_property(d, a.e, 0, 0);
        let _g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        effect::set_operation(d, a.e, Some(operation(thop)));
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    } else {
        effect::set_category(d, a.e, CATEGORY_SUMMON);
        effect::set_property(d, a.e, 0, 0);
        effect::set_operation(d, a.e, Some(operation(nsop)));
        duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    }
    true
}
fn nsop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&nsfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        duel::summon(d, a.tp, tc, true, None, 0, None);
    }
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
