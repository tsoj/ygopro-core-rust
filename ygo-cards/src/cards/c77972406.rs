//! Ambush Fangs
use super::prelude::*;

pub const ID: u32 = 77972406;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VENOM],
    counter_place_list: &[COUNTER_VENOM],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttackTarget()
    // return tc and tc:IsControler(tp) and tc:IsFaceup() and tc:IsSetCard(SET_VENOM)
    match duel::get_attack_target(d) {
        Some(tc) => card::is_controler(d, tc, a.tp) && card::is_faceup(d, tc) && card::is_set_card(d, tc, &[SET_VENOM as u16]),
        None => false,
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return tg:IsOnField() and tg:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e));
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.nil());
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.NegateAttack() then
    if card::is_relate_to_effect(d, tc, a.e) && duel::negate_attack(d) {
        // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // tc:AddCounter(COUNTER_VENOM,1)
        card::add_counter(d, tc, COUNTER_VENOM as u16, 1, false);
        // if atk>0 and tc:GetAttack()==0 then
        if atk > 0 && card::get_attack(d, tc) == 0 {
            // Duel.RaiseEvent(tc,EVENT_CUSTOM+54306223,e,0,0,0,0)
            duel::raise_event(d, CardSet::from_card(tc), EVENT_CUSTOM + 54306223, Some(a.e), 0, 0, 0, 0);
        }
    }
}
