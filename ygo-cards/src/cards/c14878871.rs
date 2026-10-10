//! Rescue Cat
use super::prelude::*;

pub const ID: u32 = 14878871;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 Level 3 or lower Beast monsters from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 3) && card::is_race(d, c, RACE_BEAST) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, c.into(), None, None, None).0 > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    if sg.len() != 2 {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    let fid = card::get_field_id(d, c);
    for tc in group::iter(d, &sg) {
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // Negate its effects
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);
    }
    duel::special_summon_complete(d);
    // sg:KeepAlive() does nothing in Rust
    // Destroy them during the End Phase
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_label(d, e3, &[fid as i64]);
    effect::set_label_object(d, e3, Value::Group(sg.clone()));
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_operation(d, e3, Some(operation(desop)));
    duel::register_effect(d, e3, tp);
}
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let g = effect::get_label_object(d, a.e);
    let g = g.as_group().nil().clone();
    let label = effect::get_label(d, a.e);
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, label), 1, NO_EX) {
        // g:DeleteGroup() does nothing in Rust
        effect::reset(d, a.e);
        return false;
    }
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = effect::get_label_object(d, a.e);
    let g = g.as_group().nil().clone();
    let label = effect::get_label(d, a.e);
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, label), NO_EX);
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
