//! Malice Doll of Demise
use super::prelude::*;

pub const ID: u32 = 72657739;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Register if this card is sent to the GY by a Continuous Spell effect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_EFFECT) {
        if let Some(re) = a.re {
            if effect::is_spell_effect(d, re) {
                let re_h = effect::get_handler(d, re).nil();
                if card::is_continuous_spell(d, re_h) {
                    let e1 = effect::create_effect(d, c);
                    effect::set_description(d, e1, aux::stringid(ID, 0));
                    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
                    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
                    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
                    effect::set_count_limit(d, e1, 1, 0, 0, 0);
                    effect::set_range(d, e1, LOCATION_GRAVE);
                    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
                    effect::set_target(d, e1, Some(cost_target(sptg)));
                    effect::set_operation(d, e1, Some(operation(spop)));
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
                    card::register_effect(d, c, e1, false);
                }
            }
        }
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
