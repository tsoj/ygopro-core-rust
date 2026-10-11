//! Genomix Fighter
use super::prelude::*;

pub const ID: u32 = 42155488;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with no tribute
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    effect::set_operation(d, e1, Some(lua_fn(ntop)));
    card::register_effect(d, c, e1, false);
    // declear
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(dectg)));
    effect::set_operation(d, e2, Some(operation(decop)));
    card::register_effect(d, c, e2, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let minc = a.int(2);
    let p = card::get_controler(d, c);
    minc == 0 && card::get_level(d, c) > 4 && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.ntop(e,tp,eg,ep,ev,re,r,rp,c)`
fn ntop(d: &mut Duel, a: Args) {
    let c = a.c(8);
    // change base attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    effect::set_value(d, e1, 1100);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
    effect::set_value(d, e2, 3);
    card::register_effect(d, c, e2, false);
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::get_race(d, c) as i64 != effect::get_label(d, e)
}
fn dectg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RACE as u64);
    let rc = duel::announce_race(d, tp, 1, RACE_ALL as u64);
    effect::set_label(d, a.e, &[rc as i64]);
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(sumlimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_label(d, e1, &[rc as i64]);
    duel::register_effect(d, e1, tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    duel::register_effect(d, e2, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.syncheck(e,c)`
fn syncheck(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::assume_property(d, c, ASSUME_RACE, effect::get_label(d, e) as u64);
    true
}
fn decop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let rc = effect::get_label(d, a.e);
        card::set_hint(d, c, CHINT_RACE as u8, rc as u64);
        let e1 = effect::create_effect(d, c);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SYNCHRO_CHECK);
        effect::set_value_fn(d, e1, effect_card_fn(syncheck));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_label(d, e1, &[rc]);
        card::register_effect(d, c, e1, false);
    }
}
