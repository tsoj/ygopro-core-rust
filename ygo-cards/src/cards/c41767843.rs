//! Score the Melodious Diva
use super::prelude::*;

pub const ID: u32 = 41767843;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MELODIOUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkdef 0
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let mut a_c = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    // if not d then return false end
    let mut d_c = match duel::get_attack_target(d) {
        Some(dd) => dd,
        None => return false,
    };
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, a_c, 1 - a.tp) {
        std::mem::swap(&mut a_c, &mut d_c);
    }
    // return a:IsSetCard(SET_MELODIOUS) and a:IsRelateToBattle() and (d:GetAttack()>0 or d:GetDefense()>0)
    card::is_set_card(d, a_c, &[SET_MELODIOUS as u16])
        && card::is_relate_to_battle(d, a_c)
        && (card::get_attack(d, d_c) > 0 || card::get_defense(d, d_c) > 0)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let mut dd = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then d=a end
    if card::is_controler(d, att, 1 - a.tp) {
        dd = Some(att);
    }
    // if not d:IsRelateToBattle() or d:IsFacedown() then return end
    let dd = dd.nil();
    if !card::is_relate_to_battle(d, dd) || card::is_facedown(d, dd) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // d:RegisterEffect(e1)
    card::register_effect(d, dd, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
    effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
    // d:RegisterEffect(e2)
    card::register_effect(d, dd, e2, false);
}
