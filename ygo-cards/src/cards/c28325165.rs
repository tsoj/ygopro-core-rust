//! Aqua Story - Urashima
use super::prelude::*;

pub const ID: u32 = 28325165;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AQUAACTRESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted monster has its effects negate, its ATK/DEF becomes 100, and is unaffected by opponent's card effects
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_AQUAACTRESS) and c:IsMonster()
    card::is_set_card(d, c, &[SET_AQUAACTRESS as u16]) && card::is_monster(d, c)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.StatChangeDamageStepCondition() and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,nil)
    aux::stat_change_damage_step_condition(d) && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // Negate its effects
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        // ATK/DEF becomes 100
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e3, 100);
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e3, false);
        let e4 = effect::clone(d, e3);
        effect::set_code(d, e4, EFFECT_SET_DEFENSE_FINAL);
        card::register_effect(d, tc, e4, false);
        // Unaffected by opponent's card effects
        let e5 = effect::create_effect(d, c);
        effect::set_description(d, e5, 3110);
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_range(d, e5, LOCATION_MZONE);
        effect::set_code(d, e5, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e5, lua_fn(efilter));
        effect::set_reset(d, e5, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e5, false);
    }
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwnerPlayer()~=e:GetOwnerPlayer()
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_owner_player(d, a.e(0))
}
