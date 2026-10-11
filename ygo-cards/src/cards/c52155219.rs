//! Salamangreat Circle
use super::prelude::*;

pub const ID: u32 = 52155219;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && immfilter(d, chkc);
    }
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    let b2 = duel::is_existing_target(d, Some(&immfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]).unwrap_or(0);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        effect::set_property(d, a.e, 0, 0);
        effect::set_operation(d, a.e, Some(operation(thop)));
    } else {
        effect::set_category(d, a.e, 0);
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        effect::set_operation(d, a.e, Some(operation(immop)));
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
        duel::select_target(d, a.tp, Some(&immfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    }
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.immfilter(c)`
fn immfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_link_monster(d, c) && card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_reincarnation_summoned(d, c)
}
/// `s.immop(e,tp,eg,ep,ev,re,r,rp)`
fn immop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // Unaffected by monster effects
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, 3101);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e1, lua_fn(efilter));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.efilter(e,te)`: `return te:IsMonsterEffect() and te:GetOwner()~=e:GetOwner()`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::is_monster_effect(d, te) && effect::get_owner(d, te) != effect::get_owner(d, e)
}
