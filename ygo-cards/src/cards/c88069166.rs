//! Odin's Eye
use super::prelude::*;

pub const ID: u32 = 88069166;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AESIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // confirm
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // cancel target
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_operation(d, e3, Some(operation(ctarget)));
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AESIR as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_chain_limit(d, lua_fn(chlimit));
    true
}
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    // function(e, ep, tp) return tp == ep end
    a.p(2) == a.p(1)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_disabled(d, tc) {
            card::set_card_target(d, c, tc);
            effect::set_label_object(d, a.e, Value::Card(tc));
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            effect::set_range(d, e1, LOCATION_MZONE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            effect::set_condition(d, e1, Some(condition(rcon)));
            card::register_effect(d, tc, e1, false);
            if card::is_immune_to_effect(d, tc, e1) {
                return;
            }
            let mut g1 = duel::get_field_group(d, tp, 0, LOCATION_HAND);
            let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
            group::add_card(&mut g1, &g2);
            duel::confirm_cards(d, tp, g1);
            duel::shuffle_hand(d, 1 - tp);
        }
    }
}
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h)
}
fn ctarget(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, h) {
        card::cancel_card_target(d, h, tc);
    }
}
