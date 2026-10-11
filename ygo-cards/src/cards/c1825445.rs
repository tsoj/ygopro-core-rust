//! Super Quantal Union - Magnaformation
use super::prelude::*;

pub const ID: u32 = 1825445;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPER_QUANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Cannot target "Super Quant" cards on the field
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, LOCATION_ONFIELD);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, cc| card::is_set_card(d, cc, &[SET_SUPER_QUANT as u16]))));
    effect::set_condition(d, e2, Some(condition(tgcon)));
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
    // Attach 1 monster to "Super Quantal Mech Beast"
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_target(d, e3, Some(cost_target(mattg)));
    effect::set_operation(d, e3, Some(operation(matop)));
    card::register_effect(d, c, e3, false);
}
/// `s.tgcon(e)`: "Check if it's your main phase"
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_turn_player(d) == effect::get_handler_player(d, a.e) && duel::is_phase(d, PHASE_MAIN1)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.filter(c,tp)`: "Check for 'Super Quant' Xyz monster"
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SUPER_QUANT as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.mattg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`: "Activation legality"
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // `chkc~=cc`: `cc` is never assigned in the script, so it is nil and the test always passes
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let _tg = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.matop(e,tp,eg,ep,ev,re,r,rp)`: "Performing the effect of attaching 1 other monster as material"
fn matop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
        let sel = duel::select_matching_card(d, tp, Some(&filter2), tp, LOCATION_MZONE, 0, 1, 1, false, tc.into()).nil();
        if let Some(mat) = group::get_first(d, &sel) {
            duel::overlay(d, tc, CardSet::from_card(mat), true);
        }
    }
}
