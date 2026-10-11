//! Dark World Brainwashing
use super::prelude::*;

pub const ID: u32 = 10131855;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Change opponent's monster effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_SZONE);
    card::register_effect(d, c, e3, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) >= 3
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARK_WORLD as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, a.tp, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_HAND) {
        let g = CardSet::new();
        duel::change_target_card(d, a.ev as u8, &g);
        duel::change_chain_operation(d, a.ev as u8, operation(repop));
    }
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    if !g.is_empty() {
        let sg = group::random_select(d, &g, 1 - a.tp, 1);
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
