//! Alien Kid
//! エーリアン・キッズ
use super::prelude::*;

pub const ID: u32 = 64160836;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALIEN],
    counter_place_list: &[COUNTER_A],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(condition(adcon)));
    // e2:SetTarget(s.adtg)
    effect::set_target(d, e2, Some(effect_card_fn(adtg)));
    // e2:SetValue(s.adval)
    effect::set_value_fn(d, e2, effect_card_value(adval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsFaceup() and tc:IsControler(1-tp) then tc:AddCounter(COUNTER_A,1) end
        if card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - a.tp) {
            card::add_counter(d, tc, COUNTER_A as u16, 1, false);
        }
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
