//! Iron Chain Dragon
use super::prelude::*;

pub const ID: u32 = 19974580;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IRON_CHAIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1 or more non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Gains 200 ATK for each "Iron Chain" monster banished
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(attg)));
    effect::set_operation(d, e1, Some(operation(atop)));
    card::register_effect(d, c, e1, false);
    // Send the top 3 cards of your opponent's Deck to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DECKDES);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(ddcon)));
    effect::set_target(d, e2, Some(cost_target(ddtg)));
    effect::set_operation(d, e2, Some(operation(ddop)));
    card::register_effect(d, c, e2, false);
}
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_IRON_CHAIN as u16]) && card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    if duel::is_player_affected_by_effect(d, a.tp, CARD_SPIRIT_ELIMINATION).is_some() {
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_MZONE as i32);
    } else {
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    }
    true
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&rfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let ct = duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    if ct > 0 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, (ct * 200) as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn ddcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp
}
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 3);
    true
}
fn ddop(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, 1 - a.tp, 3, REASON_EFFECT);
}
