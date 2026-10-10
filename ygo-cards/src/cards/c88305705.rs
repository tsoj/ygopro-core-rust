//! Brave-Eyes Pendulum Dragon
use super::prelude::*;

pub const ID: u32 = 88305705;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PENDULUM_DRAGON],
    material_setcode: &[SET_PENDULUM, SET_PENDULUM_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PENDULUM_DRAGON),aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_PENDULUM_DRAGON as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, Some(fc), st, Some(tp)))),
        ],
    );
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // remove
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_REMOVE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e3, Some(condition(rmcon)));
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    effect::set_operation(d, e3, Some(operation(rmop)));
    card::register_effect(d, c, e3, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, c) == SUMMON_TYPE_FUSION
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::get_attack(d, c) > 0
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&atkfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(ftarget)));
    effect::set_label(d, e2, &[card::get_field_id(d, c) as i64]);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
}
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && card::get_attack(d, effect::get_handler(d, re).nil()) == 0
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    effect::set_label_object(d, a.e, bc.map_or(Value::Nil, Value::Card));
    if Some(c) != duel::get_attacker(d) {
        return false;
    }
    match bc {
        Some(bc) => card::is_status(d, c, STATUS_OPPO_BATTLE) && card::is_on_field(d, bc) && card::is_relate_to_battle(d, bc),
        None => false,
    }
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let lo = effect::get_label_object(d, a.e).as_card();
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, lo.nil(), None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, lo.map(CardSet::from_card), 1, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        duel::remove(d, CardSet::from_card(bc), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
