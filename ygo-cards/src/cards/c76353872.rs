//! Defcon Bird
//! scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 76353872;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);

    // DEF
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_POSITION);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(deftg)));
    effect::set_operation(d, e2, Some(operation(defop)));
    card::register_effect(d, c, e2, false);
}

fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_CYBERSE as u64) && card::is_discardable(d, c, None, None)
}

fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, h) = (a.tp, effect::get_handler(d, a.e).nil());
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, h.into());
    }
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, h.into());
    true
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}

fn deftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let at = duel::get_attack_target(d);
    if a.chk == Some(0) {
        return match at {
            Some(at) => card::is_controler(d, at, tp) && card::is_race(d, at, RACE_CYBERSE as u64) && card::is_faceup(d, at),
            None => false,
        };
    }
    let at = at.nil();
    duel::set_target_card(d, at);
    let half = lua_int(card::get_base_attack(d, at) as f64 / 2.0) as i32;
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(at)), 1, tp, half);
    let half = lua_int(card::get_base_attack(d, at) as f64 / 2.0) as i32;
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(CardSet::from_card(at)), 1, tp, half);
    true
}

fn defop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let at = duel::first_target(d);
    let at = match at {
        Some(at) if card::is_relate_to_effect(d, at, e) && !card::is_facedown(d, at) => at,
        _ => return,
    };
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SET_DEFENSE_FINAL);
    let base = card::get_base_attack(d, at) as i64;
    effect::set_value(d, e1, base * 2);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    card::register_effect(d, at, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
    card::register_effect(d, at, e2, false);
    if !card::is_immune_to_effect(d, at, e1)
        && !card::is_immune_to_effect(d, at, e2)
        && card::is_attack_pos(d, at)
        && card::is_can_change_position(d, at)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::break_effect(d);
        duel::change_position(d, at, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
