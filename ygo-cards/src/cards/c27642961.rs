//! SPYRAL MISSION - Assault
use super::prelude::*;

pub const ID: u32 = 27642961;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPYRAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "SPYRAL" monster from your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    // Destroy this card during your 3rd End Phase after activation
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 3);
    card::set_turn_counter(d, c, 0);
    card::register_effect(d, c, e1, false);
    true
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_turn_counter(d, c) + 1;
    card::set_turn_counter(d, c, ct as u16);
    if ct == 3 {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_EFFECT)
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let des = group::get_first(d, &eg).nil();
    if card::is_reason(d, des, REASON_BATTLE) {
        let rc = card::get_reason_card(d, des);
        return match rc {
            Some(rc) => card::is_set_card(d, rc, &[SET_SPYRAL as u16]) && card::is_controler(d, rc, tp) && card::is_relate_to_battle(d, rc),
            None => false,
        };
    } else if let Some(re) = a.re {
        let rc = effect::get_handler(d, re);
        return group::is_exists(d, &eg, &|d, c| cfilter(d, c), 1, NO_EX)
            && match rc {
                Some(rc) => card::is_set_card(d, rc, &[SET_SPYRAL as u16]) && card::is_controler(d, rc, tp),
                None => false,
            }
            && effect::is_monster_effect(d, re);
    }
    false
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SPYRAL as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
