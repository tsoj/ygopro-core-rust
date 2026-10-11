//! Morphtronic Rusty Engine
use super::prelude::*;

pub const ID: u32 = 20686759;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_MORPHTRONIC}
    listed_series: &[SET_MORPHTRONIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_MORPHTRONIC))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec and ec:IsReason(REASON_DESTROY)
    card::is_reason(d, c, REASON_LOST_TARGET) && ec.is_some_and(|ec| card::is_reason(d, ec, REASON_DESTROY))
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dam=e:GetHandler():GetPreviousEquipTarget():GetBaseAttack()
    let h = effect::get_handler(d, a.e).nil();
    let mut dam = card::get_base_attack(d, card::get_previous_equip_target(d, h).nil()) as i64;
    // if dam<0 then dam=0 end
    if dam < 0 {
        dam = 0;
    }
    // e:SetLabel(dam)
    effect::set_label(d, a.e, &[dam]);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, dam as i32);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,e:GetLabel(),REASON_EFFECT,true)
    let label = effect::get_label(d, a.e);
    duel::damage(d, 1 - a.tp, label, REASON_EFFECT, true, None);
    // Duel.Damage(tp,e:GetLabel(),REASON_EFFECT,true)
    let label = effect::get_label(d, a.e);
    duel::damage(d, a.tp, label, REASON_EFFECT, true, None);
    // Duel.RDComplete()
    duel::rd_complete(d);
}
