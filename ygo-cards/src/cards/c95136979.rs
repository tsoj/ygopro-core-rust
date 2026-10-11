//! Unchained Ogre Shara
use super::prelude::*;

pub const ID: u32 = 95136979;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During the Main Phase (Quick Effect): You can discard this card; Special Summon 1 Fiend monster from your hand, then destroy 1 card you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(handsptg)));
    effect::set_operation(d, e1, Some(operation(handspop)));
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    card::register_effect(d, c, e1, false);
    // If a card(s) on the field is destroyed by card effect, except "Unchained Ogre Shara", or by battle, while this card is in your GY: You can Special Summon this card (but place it on the bottom of the Deck when it leaves the field) or add it to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY | EFFECT_FLAG_DAMAGE_STEP, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(selfspthcon)));
    effect::set_target(d, e2, Some(cost_target(selfspthtg)));
    effect::set_operation(d, e2, Some(operation(selfspthop)));
    card::register_effect(d, c, e2, false);
}
fn handspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_FIEND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| handspfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    let g = duel::get_field_group(d, tp, LOCATION_ONFIELD, 0);
    if !g.is_empty() {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    }
    true
}
fn handspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| handspfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
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
fn selfspthconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn selfspthcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    if group::is_contains(&eg, c) || !group::is_exists(d, &eg, &|d, x| selfspthconfilter(d, x), 1, NO_EX) {
        return false;
    }
    if a.r & REASON_BATTLE > 0 {
        return true;
    }
    if duel::is_chain_solving(d) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) && card::is_faceup(d, rc) {
            !card::is_code(d, rc, &[ID])
        } else {
            let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
            info[0].int() != ID as i64 && info[1].int() != ID as i64
        }
    } else if let Some(re) = a.re {
        let rc = effect::get_handler(d, re).nil();
        !card::is_code(d, rc, &[ID])
    } else {
        let first = group::get_first(d, &eg).nil();
        !card::is_previous_code_on_field(d, first, &[ID])
    }
}
fn selfspthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            || (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None));
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn selfspthop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    aux::to_hand_or_else(
        d,
        c,
        tp,
        Some(&|d, _c| {
            duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        }),
        Some(&|d, _g| {
            if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // Place it on the bottom of the Deck when it leaves the field
                let e1 = effect::create_effect(d, c);
                effect::set_description(d, e1, aux::stringid(ID, 2));
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
                effect::set_value(d, e1, LOCATION_DECKBOT as i64);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
                card::register_effect(d, c, e1, true);
            }
            0
        }),
        Some(aux::stringid(ID, 3)),
    );
}
