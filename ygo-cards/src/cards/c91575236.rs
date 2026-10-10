//! Immortal Dragon
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 91575236;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // 1 Tuner + 1+ non-Tuner monsters
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Send a Zombie to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_LVCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
    // Special Summon self
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn tgfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    let diff = (card::get_original_level(d, ec) as i64 - card::get_original_level(d, c) as i64).abs();
    card::is_race(d, c, RACE_ZOMBIE) && diff > 0 && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, h)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| tgfilter(d, x, c)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    if let Some(tc) = tc {
        if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_GRAVE) {
            // Change Level
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_value(d, e1, (card::get_original_level(d, c) as i64 - card::get_original_level(d, tc) as i64).abs());
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            card::register_effect(d, c, e1, false);
        }
    }
}
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_controler(d, c, tp) && (card::get_previous_race_on_field(d, c) & RACE_ZOMBIE) == RACE_ZOMBIE
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    match a.eg.clone() {
        None => false,
        Some(eg) => !eg.contains(c) && group::is_exists(d, &eg, &|d, x| spconfilter(d, x, a.tp), 1, c.into()),
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, true);
    }
}
