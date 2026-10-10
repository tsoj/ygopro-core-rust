//! Stygian Street Patrol
use super::prelude::*;

pub const ID: u32 = 13521194;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_condition(d, e1, Some(aux::bdgcon()));
    effect::set_target(d, e1, Some(cost_target(damtarget)));
    effect::set_operation(d, e1, Some(operation(damoperation)));
    card::register_effect(d, c, e1, false);
    // Special summon
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptarget)));
    effect::set_operation(d, e2, Some(operation(spoperation)));
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end ... end`
fn damtarget(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let atk = duel::get_attacker(d);
    let def = duel::get_attack_target(d);
    let h = effect::get_handler(d, a.e).nil();
    let m: i64 = if atk == Some(h) {
        card::get_level(d, def.nil()) as i64 * 100
    } else {
        card::get_level(d, atk.nil()) as i64 * 100
    };
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, m as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, m as i32);
    true
}
fn damoperation(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let atk = card::get_attack(d, c);
    atk >= 0 && atk <= 2000 && card::is_race(d, c, RACE_FIEND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptarget(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spoperation(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if group::get_count(&g) > 0 {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
