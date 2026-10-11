//! Morphtransition
use super::prelude::*;

pub const ID: u32 = 92890308;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_MORPHTRONIC}
    listed_series: &[SET_MORPHTRONIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let td = duel::get_attack_target(d).nil();
    // return d:IsControler(tp) and d:IsFaceup() and d:IsSetCard(SET_MORPHTRONIC)
    card::is_controler(d, td, a.tp) && card::is_faceup(d, td) && card::is_set_card(d, td, &[SET_MORPHTRONIC as u16])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ta=Duel.GetAttacker()
    let ta = duel::get_attacker(d);
    // local td=Duel.GetAttackTarget()
    let td = duel::get_attack_target(d);
    // if chkc then return chkc==ta end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == ta;
    }
    // if chk==0 then return ta:IsOnField() and ta:IsCanBeEffectTarget(e)
    //   and td:IsOnField() and td:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        return card::is_on_field(d, ta.nil())
            && card::is_can_be_effect_target(d, ta.nil(), Some(a.e))
            && card::is_on_field(d, td.nil())
            && card::is_can_be_effect_target(d, td.nil(), Some(a.e));
    }
    // local g=Group.FromCards(ta,td)
    let mut g = CardSet::new();
    if let Some(ta) = ta {
        group::add_card(&mut g, &CardSet::from_card(ta));
    }
    if let Some(td) = td {
        group::add_card(&mut g, &CardSet::from_card(td));
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,td,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, td.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ta=Duel.GetAttacker()
    let ta = duel::get_attacker(d);
    // local td=Duel.GetAttackTarget()
    let td = duel::get_attack_target(d);
    // if ta:IsRelateToEffect(e) and Duel.NegateAttack() and td:IsFaceup() and td:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, ta.nil(), a.e) && duel::negate_attack(d) && card::is_faceup(d, td.nil()) && card::is_relate_to_effect(d, td.nil(), a.e) {
        // Duel.ChangePosition(td,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)
        duel::change_position(d, td.nil(), POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
    }
}
