//! Wind Pegasus @Ignister
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 98506199;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // to deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e2, LOCATION_MZONE | LOCATION_GRAVE);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_condition(d, e2, Some(condition(tdcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn faceup_ignister(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_IGNISTER as u16])
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&faceup_ignister), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let ct = duel::get_matching_group_count(d, Some(&faceup_ignister), tp, LOCATION_MZONE, 0, NO_EX);
    if ct > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let g = duel::select_matching_card(d, tp, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::get_previous_controler(d, c) == tp
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::get_reason_player(d, c) == 1 - tp
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, x| cfilter(d, x, tp), 1, c.into())
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_able_to_deck(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_able_to_deck), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&card::is_able_to_deck), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n as u8, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
