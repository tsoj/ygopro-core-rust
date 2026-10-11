//! Wave-Motion Inferno
use super::prelude::*;

pub const ID: u32 = 63665606;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk,def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.atcon)
    effect::set_condition(d, e2, Some(lua_fn(atcon)));
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_INFERNITY))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_INFERNITY as u16]))));
    // e2:SetValue(400)
    effect::set_value(d, e2, 400);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // discard
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e4, Some(cost::self_to_grave()));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_HAND,0)==0
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) == 0
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,0,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 0, LOCATION_HAND as i32);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
