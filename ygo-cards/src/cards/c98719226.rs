//! Alien Warrior
use super::prelude::*;

pub const ID: u32 = 98719226;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALIEN as u32],
    counter_place_list: &[COUNTER_A as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counter
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(ctcon)));
    effect::set_operation(d, e1, Some(operation(ctop)));
    card::register_effect(d, c, e1, false);
    // atk def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(adcon)));
    effect::set_target(d, e2, Some(effect_card_fn(adtg)));
    effect::set_value_fn(d, e2, effect_card_value(adval));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetReasonCard()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_reason_card(d, c).nil();
    // if tc:IsFaceup() and tc:IsRelateToBattle() then
    if card::is_faceup(d, tc) && card::is_relate_to_battle(d, tc) {
        // tc:AddCounter(COUNTER_A,2)
        card::add_counter(d, tc, COUNTER_A as u16, 2, false);
    }
}
/// `s.adcon(e)`
fn adcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and Duel.GetAttackTarget()
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()
}
/// `s.adtg(e,c)`
fn adtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // local bc=c:GetBattleTarget()
    // return bc and c:GetCounter(COUNTER_A)~=0 and bc:IsSetCard(SET_ALIEN)
    match card::get_battle_target(d, c) {
        Some(bc) => card::get_counter(d, c, COUNTER_A as u16) != 0 && card::is_set_card(d, bc, &[SET_ALIEN as u16]),
        None => false,
    }
}
/// `s.adval(e,c)`
fn adval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(COUNTER_A)*-300
    card::get_counter(d, c, COUNTER_A as u16) as i64 * -300
}
