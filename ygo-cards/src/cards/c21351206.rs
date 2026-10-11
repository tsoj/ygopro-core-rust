//! Infernoble Knight Ogier
use super::prelude::*;

pub const ID: u32 = 21351206;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_NOBLE_ARMS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Upon normal summon, send 1 FIRE warrior or "Noble Arm" card from deck to GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
    // Same as above, but on special summon
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // If in the GY, equip this card to a warrior monster
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e3, CATEGORY_EQUIP);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    effect::set_operation(d, e3, Some(operation(eqop)));
    card::register_effect(d, c, e3, false);
    // A monster equipped with this card cannot be destroyed by card effect
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value(d, e4, 1);
    card::register_effect(d, c, e4, false);
}
/// Check for FIRE warrior monster (except for this card's name) or "Noble Arms" card
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_race(d, c, RACE_WARRIOR) && card::is_attribute(d, c, ATTRIBUTE_FIRE)) || card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]))
        && card::is_able_to_grave(d, c, None)
        && !card::is_code(d, c, &[ID])
}
/// Activation legality
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// Send 1 FIRE warrior or "Noble Arm" card from deck to GY
fn tgop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0 {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// Check for warrior monster
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR)
}
/// Activation legality
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// If in the GY, equip this card to a warrior monster
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, a.tp) && !card::is_facedown(d, tc) {
                duel::equip(d, a.tp, c, tc, true, false);
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
                effect::set_label_object(d, e1, Value::Card(tc));
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
/// Target is valid
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
