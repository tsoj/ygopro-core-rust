//! Battlewasp - Dart the Hunter
use super::prelude::*;

pub const ID: u32 = 91283212;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BATTLEWASP],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(damcon)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(damtg)));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
    // tuner
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(tntg)));
    effect::set_operation(d, e2, Some(operation(tnop)));
    card::register_effect(d, c, e2, false);
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    let bc_opt = card::get_battle_target(d, tc);
    card::is_relate_to_battle(d, tc)
        && card::is_status(d, tc, STATUS_OPPO_BATTLE)
        && card::is_controler(d, tc, a.tp)
        && card::is_set_card(d, tc, &[SET_BATTLEWASP as u16])
        && {
            let bc = bc_opt.nil();
            card::is_location(d, bc, LOCATION_GRAVE)
                && card::is_reason(d, bc, REASON_BATTLE)
                && card::get_previous_controler(d, bc) != a.tp
                && card::get_base_attack(d, bc) > 0
                && card::get_owner(d, bc) == 1 - a.tp
        }
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    let bc = card::get_battle_target(d, tc).nil();
    let mut atk = card::get_base_attack(d, bc);
    if atk < 0 {
        atk = 0;
    }
    duel::set_target_param(d, atk as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
// function s.tnfilter(c)
fn tnfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TUNER) && card::is_race(d, c, RACE_INSECT)
}
// function s.tntg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tntg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, a.tp)
            && tnfilter(d, chkc)
            && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tnfilter), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&tnfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
// function s.tnop(e,tp,eg,ep,ev,re,r,rp)
fn tnop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let e0 = effect::create_effect(d, c);
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e0, EFFECT_ADD_TYPE);
        effect::set_reset(d, e0, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value(d, e0, TYPE_TUNER as i64);
        card::register_effect(d, tc, e0, false);
    }
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // lizard check
    aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_race(d, c, RACE_INSECT) && card::is_location(d, c, LOCATION_EXTRA)
}
// function s.lizfilter(e,c)
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_original_race(d, c, RACE_INSECT)
}
