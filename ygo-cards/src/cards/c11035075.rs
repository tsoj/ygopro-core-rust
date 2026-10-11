//! Shichi-Go-San
use super::prelude::*;

pub const ID: u32 = 11035075;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Level/Rank 1 monster from either GY to your field, and if you do, equip this card to it
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // Increase the equipped monster's Level/Rank by 1
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, c, ID, None)
    })));
    effect::set_operation(d, e2, Some(operation(lvrkop)));
    card::register_effect(d, c, e2, false);
    // 3+: It gains ATK equal to its original ATK
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e3, Some(condition(|d, a| effcon(d, a, 3))));
    effect::set_value_fn(d, e3, effect_card_value(|d, _e, c| card::get_base_attack(d, c) as i64));
    card::register_effect(d, c, e3, false);
    // 5+: It is unaffected by your opponent's activated effects
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
    effect::set_condition(d, e4, Some(condition(|d, a| effcon(d, a, 5))));
    effect::set_value_fn(d, e4, lua_fn(|d, a: Args| {
        let e = a.e(0);
        let te = a.e(1);
        effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e) && effect::is_activated(d, te)
    }));
    card::register_effect(d, c, e4, false);
    // Send all cards on the field to the GY
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 2));
    effect::set_category(d, e5, CATEGORY_TOGRAVE);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_PHASE + PHASE_STANDBY);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(|d, a| duel::is_turn_player(d, a.tp) && effcon(d, a, 7))));
    effect::set_target(d, e5, Some(cost_target(tgtg)));
    effect::set_operation(d, e5, Some(operation(tgop)));
    card::register_effect(d, c, e5, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    (card::is_level(d, c, &[1]) || card::is_rank(d, c, &[1])) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        card::cancel_to_grave(d, c, false);
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| spfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
            && card::is_relate_to_effect(d, c, a.e)
            && !card::is_status(d, c, STATUS_LEAVE_CONFIRMED)
            && duel::equip(d, a.tp, c, sc, true, false)
        {
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            // Equip limit
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            effect::set_value_fn(d, e1, effect_card_fn(move |_d, _e, cc| cc == sc));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e1, false);
            return;
        }
    }
    card::cancel_to_grave(d, c, false);
}
fn lvrkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c).nil();
    let eff_code = if card::has_level(d, ec) {
        EFFECT_UPDATE_LEVEL
    } else if card::has_rank(d, ec) {
        EFFECT_UPDATE_RANK
    } else {
        0
    };
    // Increase the equipped monster's Level/Rank by 1
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, eff_code);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, ec, e1, false);
}
/// `s.effcon(level_rank)(e)`
fn effcon(d: &mut Duel, a: &EffArgs, level_rank: u32) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, c, ID, None) && {
        let ec = card::get_equip_target(d, c).nil();
        card::is_level_above(d, ec, level_rank) || card::is_rank_above(d, ec, level_rank)
    }
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        return ct > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, ct as u8, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
