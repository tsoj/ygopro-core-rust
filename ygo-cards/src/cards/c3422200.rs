//! Crystron Sulfefnir
use super::prelude::*;

pub const ID: u32 = 3422200;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_CRYSTRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card in Defense Position, then destroy 1 card you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::discard(Some(std::sync::Arc::new(|d, c, _e, _tp| {
        card::is_set_card(d, c, &[SET_CRYSTRON as u16]) && !card::is_code(d, c, &[ID])
    })), true, None, None, None)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Crystron" monster from your Deck in Defense Position
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(decksptg)));
    effect::set_operation(d, e2, Some(operation(deckspop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // r&(REASON_BATTLE|REASON_EFFECT)>0 and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    (a.r & (REASON_BATTLE | REASON_EFFECT)) > 0 && card::is_previous_location(d, effect::get_handler(d, a.e).nil(), LOCATION_ONFIELD)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, 0);
    if !g.is_empty() {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    }
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, &g, true);
            duel::break_effect(d);
            duel::destroy(d, &g, REASON_EFFECT, None, None);
        }
    }
}
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_CRYSTRON as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn deckspop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
