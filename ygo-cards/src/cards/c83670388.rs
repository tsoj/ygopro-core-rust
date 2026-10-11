//! Icejade Curse
use super::prelude::*;

pub const ID: u32 = 83670388;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[7142724], listed_series: &[SET_ICEJADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Prevent the activation of monster's effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(accon)));
    effect::set_value_fn(d, e2, lua_fn(aclim));
    card::register_effect(d, c, e2, false);
    // Inflict damage
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(damcon)));
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
}
fn faceup_icejade(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ICEJADE as u16])
}
fn accon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&faceup_icejade), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && (duel::is_environment(d, 7142724, None, None)
            || duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[7142724])),
                0,
                LOCATION_ONFIELD,
                LOCATION_ONFIELD,
                1,
                NO_EX,
            ))
}
fn aclim(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    let status = STATUS_SUMMON_TURN + STATUS_FLIP_SUMMON_TURN + STATUS_SPSUMMON_TURN;
    card::is_location(d, rc, LOCATION_MZONE) && card::is_status(d, rc, status)
}
fn damcfilter(d: &mut Duel, c: CardId, tp: u8, ct: usize) -> bool {
    if card::is_previous_controler(d, c, tp) && card::is_previous_set_card(d, c, &[SET_ICEJADE as u16]) {
        return true;
    }
    if ct != 1 {
        return false;
    }
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_controler(d, bc, tp) && card::is_set_card(d, bc, &[SET_ICEJADE as u16]),
        None => false,
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let n = eg.len();
    group::is_exists(d, &eg, &|d, c| damcfilter(d, c, a.tp, n), 1, NO_EX)
}
fn damtgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_can_be_effect_target(d, c, Some(e)) && card::get_base_attack(d, c) > 0 && !card::is_type(d, c, TYPE_TOKEN)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return a.eg.clone().nil().contains(chkc) && damtgfilter(d, chkc, a.e);
    }
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| damtgfilter(d, c, a.e), 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let eg = a.eg.clone().nil();
    let tg = group::filter_select(d, &eg, a.tp, &|d, c| damtgfilter(d, c, a.e), 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, tg.clone());
    let tc = group::get_first(d, &tg).nil();
    let atk = card::get_base_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::get_base_attack(d, tc) > 0 {
        let atk = card::get_base_attack(d, tc) as i64;
        duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None);
    }
}
