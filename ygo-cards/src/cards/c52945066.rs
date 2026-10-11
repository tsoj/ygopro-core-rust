//! Marincess Wave
use super::prelude::*;

pub const ID: u32 = 52945066;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate 1 of opponent's monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    card::register_effect(d, c, e1, false);
    // Activate it from hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    effect::set_condition(d, e2, Some(lua_fn(actcon)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_link_monster(d, c)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_negatable_monster(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.IsNegatableMonster,...): s inherits from Card (setmetatable(self_table,Card)), so this is Card.IsNegatableMonster
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_can_be_disabled_by_effect(d, tc, a.e, true) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        let g = duel::get_matching_group(d, Some(&|d, sc| card::is_faceup(d, sc)), a.tp, LOCATION_MZONE, 0, NO_EX);
        let lg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        if !g.is_empty() && group::is_exists(d, &lg, &|d, sc| card::is_link_above(d, sc, 2), 1, NO_EX) {
            duel::break_effect(d);
            for gc in group::iter(d, &g) {
                // Your monsters are unaffected by opponent's card effects
                let e3 = effect::create_effect(d, c);
                effect::set_description(d, e3, 3110);
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
                effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_range(d, e3, LOCATION_MZONE);
                effect::set_value_fn(d, e3, lua_fn(efilter));
                effect::set_owner_player(d, e3, a.tp);
                effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, gc, e3, false);
            }
        }
    }
}
/// `s.efilter(e,re)`: `return e:GetOwnerPlayer()~=re:GetOwnerPlayer()`
fn efilter(d: &mut Duel, a: Args) -> bool {
    effect::get_owner_player(d, a.e(0)) != effect::get_owner_player(d, a.e(1))
}
/// `s.actfilter(c)`
fn actfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_link_monster(d, c) && card::is_link_above(d, c, 3)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&actfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
