//! Pandemonium
use super::prelude::*;

pub const ID: u32 = 94585852;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Alter LP cost for "Archfiend" monsters during the Standby Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_code(d, e2, EFFECT_LPCOST_REPLACE);
    effect::set_condition(d, e2, Some(condition(lrcon)));
    card::register_effect(d, c, e2, false);
    // Search an "Archfiend" monster
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_EVENT_PLAYER, 0);
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    effect::set_target(d, e4, Some(cost_target(target)));
    effect::set_operation(d, e4, Some(operation(op)));
    card::register_effect(d, c, e4, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // Global effect since either player can use this effect
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        effect::set_operation(d, ge1, Some(operation(regop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.regfilter(c)`
fn regfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_reason(d, c, REASON_DESTROY)
        && !card::is_reason(d, c, REASON_BATTLE)
        && card::is_set_card(d, c, &[SET_ARCHFIEND as u16])
        && card::has_level(d, c)
        && (card::is_previous_location(d, c, LOCATION_MZONE) || (!card::is_previous_location(d, c, LOCATION_MZONE) && card::is_monster(d, c)))
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let mut lv1: u32 = 0;
    let mut lv2: u32 = 0;
    let g1 = CardSet::new();
    let _g2 = CardSet::new();
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        if regfilter(d, tc) {
            let tlv = card::get_level(d, tc);
            if card::is_controler(d, tc, 0) {
                if tlv > lv1 {
                    lv1 = tlv;
                }
            } else if tlv > lv2 {
                lv2 = tlv;
            }
        }
    }
    if lv1 > 0 {
        duel::raise_event(d, g1.clone(), EVENT_CUSTOM + ID, a.re, a.r, a.rp, 0, lv1);
    }
    if lv2 > 0 {
        duel::raise_event(d, g1, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 1, lv2);
    }
}
/// `s.lrcon(e,tp,eg,ep,ev,re,r,rp)`
fn lrcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re then return false end
    let Some(re) = a.re else { return false };
    let rc = effect::get_handler(d, re).nil();
    duel::is_phase(d, PHASE_STANDBY) && card::is_set_card(d, rc, &[SET_ARCHFIEND as u16]) && card::is_monster(d, rc)
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    (card::get_level(d, c) as i64) < lv && card::is_set_card(d, c, &[SET_ARCHFIEND as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let ev = a.ev as i64;
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ev)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)` (named `op`: `operation` is the prelude's wrapper)
fn op(d: &mut Duel, a: &EffArgs) {
    let ev = a.ev as i64;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, ev)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
