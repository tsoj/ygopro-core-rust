//! Unchained Ogre Shma
use super::prelude::*;

pub const ID: u32 = 31531914;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UNCHAINED], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Normal Summoned: You can Special Summon 1 Level 4 or lower "Unchained" monster from your Deck, except "Unchained Ogre Shma", then destroy 1 card you control, also you cannot Special Summon for the rest of this turn, except Fiend monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(lv4sptg)));
    effect::set_operation(d, e1, Some(operation(lv4spop)));
    card::register_effect(d, c, e1, false);
    // If this card on the field is destroyed by card effect, except "Unchained Ogre Shma", or by battle: You can Special Summon 1 "Unchained" monster from your hand or Deck, except "Unchained Ogre Shma"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(handdeckspcon)));
    effect::set_target(d, e2, Some(cost_target(handdecksptg)));
    effect::set_operation(d, e2, Some(operation(handdeckspop)));
    card::register_effect(d, c, e2, false);
}
fn lv4spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && unchainedspfilter(d, c, e, tp)
}
fn unchainedspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_UNCHAINED as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn lv4sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| lv4spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    let g = duel::get_field_group(d, tp, LOCATION_ONFIELD, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    true
}
fn lv4spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| lv4spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            let dg = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
            if !dg.is_empty() {
                duel::hint_selection(d, dg.clone(), true);
                duel::break_effect(d);
                duel::destroy(d, dg, REASON_EFFECT, None, None);
            }
        }
    }
    let c = effect::get_handler(d, e).nil();
    // You cannot Special Summon for the rest of this turn, except Fiend monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race_except(d, c, RACE_FIEND, None, 0, None))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // "Clock Lizard" check
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !card::is_original_race(d, c, RACE_FIEND))), None, None, None, None);
}
fn handdeckspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)) {
        return false;
    }
    if card::is_reason(d, c, REASON_BATTLE) {
        true
    } else if card::is_reason(d, c, REASON_EFFECT) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if duel::is_chain_solving(d) {
            if rc == c {
                !card::is_previous_code_on_field(d, c, &[ID])
            } else if card::is_relate_to_effect(d, rc, re) && card::is_faceup(d, rc) {
                !card::is_code(d, rc, &[ID])
            } else {
                let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
                info[0].int() != ID as i64 && info[1].int() != ID as i64
            }
        } else {
            !card::is_code(d, rc, &[ID])
        }
    } else {
        false
    }
}
fn handdecksptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| unchainedspfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn handdeckspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| unchainedspfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
