//! Doomstar Ulka
use super::prelude::*;

pub const ID: u32 = 49109013;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When exactly 1 monster you control (and no other monsters) leaves the field by an opponent's card effect (except during the Damage Step): You can banish this card from your hand or GY; if that monster is in the GY or banished face-up, Special Summon it, otherwise, Special Summon this banished card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost::self_banish()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned: You can make this card gain 1500 ATK until the end of the next turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    if !(eg.len() == 1 && a.rp == 1 - a.tp) {
        return false;
    }
    let ec = group::get_first(d, &eg).nil();
    card::is_previous_location(d, ec, LOCATION_MZONE)
        && card::is_previous_controler(d, ec, a.tp)
        && card::is_reason(d, ec, REASON_EFFECT)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg).nil();
    let sc_chk = card::is_location(d, ec, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_faceup(d, ec);
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        if sc_chk {
            return card::is_can_be_special_summoned(d, ec, a.e, 0, a.tp, false, false, None, None, None);
        } else {
            return duel::is_player_can_special_summon_monster(d, a.tp, ID, token_data(), None, None, 0);
        }
    }
    let c = effect::get_handler(d, a.e).nil();
    let sc = if sc_chk { ec } else { c };
    card::create_effect_relation(d, c, a.e);
    card::create_effect_relation(d, ec, a.e);
    effect::set_label(d, a.e, &[if sc_chk { 1 } else { 0 }]);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(sc)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ec = group::get_first(d, &a.eg.clone().nil());
    let label = effect::get_label(d, a.e);
    if label == 1 {
        let ec = ec.nil();
        if card::is_relate_to_effect(d, ec, a.e) {
            duel::special_summon(d, ec, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        } else if card::is_relate_to_effect(d, c, a.e) {
            duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    } else if label == 0 && card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, a.tp, 1500);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Make this card gain 1500 ATK until the end of the next turn
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1500);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        card::register_effect(d, c, e1, false);
    }
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(1500),
        defense: Some(400),
        level: Some(4),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_WIND),
        ..Default::default()
    }
}
