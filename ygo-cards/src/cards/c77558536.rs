//! Raiden, Hand of the Lightsworn
use super::prelude::*;

pub const ID: u32 = 77558536;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // discard deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DECKDES);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 2);
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 2);
    true
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_GRAVE) && card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c)
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, a.tp, 2, REASON_EFFECT);
    let g = duel::get_operated_group(d);
    let ct = group::filter_count(d, &g, &cfilter, NO_EX);
    if ct == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        duel::break_effect(d);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 200);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        card::register_effect(d, c, e1, false);
    }
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    a.tp == duel::get_turn_player(d)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 2);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, a.tp, 2, REASON_EFFECT);
}
