//! Alghoul Mazera
use super::prelude::*;

pub const ID: u32 = 45154513;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Substitute destruction for your Zombie monster(s)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(reptg)));
    effect::set_value_fn(d, e1, effect_card_fn(repval));
    effect::set_operation(d, e1, Some(operation(repop)));
    card::register_effect(d, c, e1, false);
    // Special Summon itself it is banished from the hand or GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}

/// `If your Zombie monster(s) would be destroyed by battle or effect`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_race(d, c, RACE_ZOMBIE)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// Activation legality
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// Check if it was in hand/GY
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_HAND | LOCATION_GRAVE)
}
/// Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// Special Summon itself
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        duel::break_effect(d);
        // Decrease Level by 1
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        effect::set_value(d, e1, -1);
        card::register_effect(d, c, e1, false);
    }
}
