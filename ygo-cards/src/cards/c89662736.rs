//! Bujingi Crow
use super::prelude::*;

pub const ID: u32 = 89662736;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BUJIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // negate attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.nacon)
    effect::set_condition(d, e1, Some(condition(nacon)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.natg)
    effect::set_target(d, e1, Some(cost_target(natg)));
    // e1:SetOperation(s.naop)
    effect::set_operation(d, e1, Some(operation(naop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.nacon(e,tp,eg,ep,ev,re,r,rp)`
fn nacon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d).nil();
    // return at:IsControler(tp) and at:IsFaceup() and at:IsSetCard(SET_BUJIN) and at:IsRace(RACE_BEASTWARRIOR)
    card::is_controler(d, at, a.tp)
        && card::is_faceup(d, at)
        && card::is_set_card(d, at, &[SET_BUJIN as u16])
        && card::is_race(d, at, RACE_BEASTWARRIOR)
}
/// `s.natg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():IsOnField() end
    if a.chk == Some(0) {
        return card::is_on_field(d, duel::get_attacker(d).nil());
    }
    // local dam=Duel.GetAttacker():GetAttack()/2
    let atk = card::get_attack(d, duel::get_attacker(d).nil());
    let dam = lua_int(atk as f64 / 2.0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
/// `s.naop(e,tp,eg,ep,ev,re,r,rp)`
fn naop(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if Duel.NegateAttack() then
    if duel::negate_attack(d) {
        // Duel.Damage(1-tp,a:GetAttack()/2,REASON_EFFECT)
        let atk = card::get_attack(d, at.nil());
        duel::damage(d, 1 - a.tp, lua_int(atk as f64 / 2.0), REASON_EFFECT, false, None);
    }
}
