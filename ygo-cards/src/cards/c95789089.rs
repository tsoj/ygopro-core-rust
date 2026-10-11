//! Kangaroo Champ
use super::prelude::*;

pub const ID: u32 = 95789089;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.cpcon)
    effect::set_condition(d, e1, Some(condition(cpcon)));
    // e1:SetTarget(s.cptg)
    effect::set_target(d, e1, Some(cost_target(cptg)));
    // e1:SetOperation(s.cpop)
    effect::set_operation(d, e1, Some(operation(cpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cpcon(e,tp,eg,ep,ev,re,r,rp)`
fn cpcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local t=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    let t = card::get_battle_target(d, h);
    // e:SetLabelObject(t)
    let label = match t {
        Some(t) => Value::Card(t),
        None => Value::Nil,
    };
    effect::set_label_object(d, e, label);
    // return t and t:IsRelateToBattle()
    match t {
        Some(t) => card::is_relate_to_battle(d, t),
        None => false,
    }
}
/// `s.cptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,e:GetLabelObject(),1,0,0)
    let t = effect::get_label_object(d, a.e).as_card().map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, t, 1, 0, 0);
    true
}
/// `s.cpop(e,tp,eg,ep,ev,re,r,rp)`
fn cpop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_card().nil();
    // if g:IsRelateToBattle() and g:IsAttackPos() then
    if card::is_relate_to_battle(d, g) && card::is_attack_pos(d, g) {
        // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
        duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
