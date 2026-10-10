//! XYZ Combine
use super::prelude::*;

pub const ID: u32 = 95471006;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Special Summon 1 "X-Head Cannon", "Y-Dragon Head", or "Z-Metal Tank" from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_REMOVE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(dspcon)));
    effect::set_target(d, e1, Some(cost_target(dsptg)));
    effect::set_operation(d, e1, Some(operation(dspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon up to 2 of your banished "X-Head Cannon", "Y-Dragon Head", or "Z-Metal Tank"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(rmspcost)));
    effect::set_target(d, e2, Some(cost_target(rmsptg)));
    effect::set_operation(d, e2, Some(operation(rmspop)));
    card::register_effect(d, c, e2, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_card_types: &[TYPE_UNION],
    listed_names: &[62651957, 65622692, 64500000],
    ..ScriptInfo::DEFAULT
};

fn dspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !(card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_type(d, c, TYPE_UNION)
        && card::is_faceup(d, c)
        && card::is_previous_controler(d, c, tp))
    {
        return false;
    }
    let prev_mzone = card::is_previous_location(d, c, LOCATION_MZONE)
        && (card::get_previous_type_on_field(d, c) & TYPE_UNION) > 0
        && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_LIGHT) > 0
        && (card::get_previous_race_on_field(d, c) & RACE_MACHINE) > 0;
    ((prev_mzone || card::is_previous_location(d, c, LOCATION_SZONE)) && card::is_previous_position(d, c, POS_FACEUP as u32))
        || !card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn dspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| dspconfilter(d, c, a.tp), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[62651957, 65622692, 64500000]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn dsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn dspop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn rmspcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_FUSION)
        && card::is_faceup(d, c)
        && card::is_able_to_extra_as_cost(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn rmspcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rmspcostfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| rmspcostfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    true
}
fn rmsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, a.e, a.tp) && card::is_faceup(d, c)),
            a.tp,
            LOCATION_REMOVED,
            0,
            1,
            NO_EX,
        );
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
fn rmspop(d: &mut Duel, a: &EffArgs) {
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft < 1 {
        return;
    }
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| spfilter(d, c, a.e, a.tp) && card::is_faceup(d, c)),
        a.tp,
        LOCATION_REMOVED,
        0,
        NO_EX,
    );
    if g.is_empty() {
        return;
    }
    ft = ft.min(2);
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let dn = aux::dncheck();
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(1),
        Some(ft as usize),
        Some(&dn),
        1,
        a.tp,
        Some(HINTMSG_SPSUMMON as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    if !sg.is_empty() {
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
