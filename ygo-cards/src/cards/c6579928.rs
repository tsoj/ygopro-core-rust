//! Secret Six Samurai - Kizaru
use super::prelude::*;

pub const ID: u32 = 6579928;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Six Samurai" monster from the Deck to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Destruction replacement for exactly 1 "Six Samurai" monster
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_target(d, e2, Some(cost_target(reptg)));
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    effect::set_operation(d, e2, Some(operation(repop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId, att: u32) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, att)
}
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && {
            let att = card::get_attribute(d, c);
            !duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, att)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None)
            && eg.len() == 1
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let tp = effect::get_handler_player(d, e);
    repfilter(d, c, tp)
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
