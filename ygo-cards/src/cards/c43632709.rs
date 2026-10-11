//! Cerynemesia, Mystical Beast of the Forest
use super::prelude::*;

pub const ID: u32 = 43632709;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Normal or Special Summoned: banish 1 Beast; Special Summon 1 EARTH Beast
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_cost(d, e1a, Some(cost_target(spcost)));
    effect::set_target(d, e1a, Some(cost_target(sptg)));
    effect::set_operation(d, e1a, Some(operation(spop)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
    // Your opponent's monsters that can attack must attack the monster you control with the highest ATK
    let e2a = effect::create_effect(d, c);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2a, EFFECT_MUST_ATTACK);
    effect::set_range(d, e2a, LOCATION_MZONE);
    effect::set_target_range(d, e2a, 0, LOCATION_MZONE);
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EFFECT_MUST_ATTACK_MONSTER);
    effect::set_value_fn(d, e2b, effect_card_fn(must_attack_value));
    card::register_effect(d, c, e2b, false);
}
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_BEAST)
        && card::has_level(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && {
            let lv = card::get_original_level(d, c) as i64;
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, lv)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_BEAST)
        && card::is_level_below(d, c, lv as u32)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcost`: the cost check (`chk == 0`) and the payment of the banish.
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let sc = duel::select_matching_card(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX)
        .nil();
    let sc = group::get_first(d, &sc).nil();
    let lv = card::get_original_level(d, sc) as i64;
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(lv));
    let in_mzone = card::is_location(d, sc, LOCATION_MZONE);
    let ret: aux::DelayedFn = if in_mzone {
        std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
            aux::default_field_return_op(d, ag);
            true
        })
    } else {
        std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
            duel::send_to_hand(d, ag, None, REASON_EFFECT, None);
            true
        })
    };
    // Banish (face-up) 1 Beast monster from your hand or face-up field until the End Phase
    aux::remove_until(d, sc, Some(POS_FACEUP as u8), REASON_COST, PHASE_END, ID, a.e, a.tp, Some(ret), None, None, None, None, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let lv = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, lv))(d, c)),
        tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() && duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let opp = 1 - tp;
        duel::shuffle_deck(d, tp);
        if duel::get_location_count(d, opp, LOCATION_MZONE as u8, Some(opp), None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, opp, false, false, None, None, None)), opp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, opp, aux::stringid(ID, 1))
        {
            duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_SPSUMMON as u64);
            let og = duel::select_matching_card(d, opp, Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, opp, false, false, None, None, None)), opp, LOCATION_HAND, 0, 1, 1, false, NO_EX)
                .nil();
            if !og.is_empty() {
                duel::break_effect(d);
                duel::special_summon(d, &og, 0, opp, opp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
/// `function(e,c) ... end` (EFFECT_MUST_ATTACK_MONSTER value)
fn must_attack_value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), p, LOCATION_MZONE, 0, NO_EX);
    let mx = group::get_max_group(d, &g, &|d, x| card::get_attack(d, x) as i64).map_or(CardSet::new(), |x| x.0);
    group::is_contains(&mx, c)
}
