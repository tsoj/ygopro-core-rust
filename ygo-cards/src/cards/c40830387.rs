//! Ancient Gear Fist
use super::prelude::*;

pub const ID: u32 = 40830387;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_GEAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_ANCIENT_GEAR))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]))), None, None, None, None, None, None);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // local dt=nil
    let mut dt: Option<CardId> = None;
    // if ec==Duel.GetAttacker() then dt=Duel.GetAttackTarget()
    if ec == duel::get_attacker(d) {
        dt = duel::get_attack_target(d);
    // elseif ec==Duel.GetAttackTarget() then dt=Duel.GetAttacker()
    } else if ec == duel::get_attack_target(d) {
        dt = duel::get_attacker(d);
    }
    // e:SetLabelObject(dt)
    effect::set_label_object(d, a.e, dt.map_or(Value::Nil, Value::Card));
    // return dt and dt:IsRelateToBattle()
    match dt {
        Some(dt) => card::is_relate_to_battle(d, dt),
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetLabelObject(),1,0,0)
    let lo = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, lo.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local dt=e:GetLabelObject()
    let dt = effect::get_label_object(d, a.e).as_card().nil();
    // if dt:IsRelateToBattle() then
    if card::is_relate_to_battle(d, dt) {
        // Duel.Destroy(dt,REASON_EFFECT)
        duel::destroy(d, dt, REASON_EFFECT, None, None);
    }
}
