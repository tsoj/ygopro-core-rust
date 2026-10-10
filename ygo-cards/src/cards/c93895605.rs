//! Hate Buster
use super::prelude::*;

pub const ID: u32 = 93895605;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // return tc:IsControler(tp) and tc:IsFaceup() and tc:IsRace(RACE_FIEND)
    card::is_controler(d, tc, a.tp) && card::is_faceup(d, tc) && card::is_race(d, tc, RACE_FIEND as u64)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local a=Duel.GetAttacker()
    // local d=Duel.GetAttackTarget()
    // if chk==0 then return a:IsOnField() and a:IsCanBeEffectTarget(e)
    //   and d:IsOnField() and d:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let atk = duel::get_attacker(d).nil();
        let dfn = duel::get_attack_target(d).nil();
        return card::is_on_field(d, atk)
            && card::is_can_be_effect_target(d, atk, Some(a.e))
            && card::is_on_field(d, dfn)
            && card::is_can_be_effect_target(d, dfn, Some(a.e));
    }
    // local g=Group.FromCards(a,d)
    let atk = duel::get_attacker(d).nil();
    let dfn = duel::get_attack_target(d).nil();
    let mut g = CardSet::from_card(atk);
    group::add_card(&mut g, &CardSet::from_card(dfn));
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dfn = duel::get_attack_target(d).nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // if a:IsRelateToEffect(e) and a:CanAttack() and not a:IsStatus(STATUS_ATTACK_CANCELED)
    //   and d:IsFaceup() and d:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, atk, a.e)
        && card::can_attack(d, atk)
        && !card::is_status(d, atk, STATUS_ATTACK_CANCELED)
        && card::is_faceup(d, dfn)
        && card::is_relate_to_effect(d, dfn, a.e)
    {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
        // if not a:IsOnField() then
        if !card::is_on_field(d, atk) {
            // Duel.Damage(1-tp,a:GetAttack(),REASON_EFFECT)
            let amount = card::get_attack(d, atk) as i64;
            duel::damage(d, 1 - a.tp, amount, REASON_EFFECT, false, None);
        }
    }
}
