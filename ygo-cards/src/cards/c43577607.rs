//! Yang Zing Prana
use super::prelude::*;

pub const ID: u32 = 43577607;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YANG_ZING], ..ScriptInfo::DEFAULT };

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
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(|d, a| effcon(d, a.e))));
    // e2:SetLabel(2)
    effect::set_label(d, e2, &[2]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // replace
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetValue(s.repval)
    effect::set_value_fn(d, e3, effect_card_value(repval));
    // e3:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(operation(repop)));
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(|d, a| effcon(d, a.e))));
    // e3:SetLabel(3)
    effect::set_label(d, e3, &[3]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot mset
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e4, EFFECT_CANNOT_MSET);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // e4:SetCondition(s.effcon)
    effect::set_condition(d, e4, Some(condition(|d, a| effcon(d, a.e))));
    // e4:SetLabel(4)
    effect::set_label(d, e4, &[4]);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // poschange
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_SET_POSITION)
    effect::set_code(d, e5, EFFECT_SET_POSITION);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    // e5:SetValue(POS_FACEUP_ATTACK)
    effect::set_value(d, e5, POS_FACEUP_ATTACK as i64);
    // e5:SetCondition(s.effcon)
    effect::set_condition(d, e5, Some(condition(|d, a| effcon(d, a.e))));
    // e5:SetLabel(4)
    effect::set_label(d, e5, &[4]);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // destroy
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e6, CATEGORY_DESTROY);
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.effcon)
    effect::set_condition(d, e6, Some(condition(|d, a| effcon(d, a.e))));
    // e6:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e6, Some(cost::self_to_grave()));
    // e6:SetTarget(s.destg)
    effect::set_target(d, e6, Some(cost_target(destg)));
    // e6:SetOperation(s.desop)
    effect::set_operation(d, e6, Some(operation(desop)));
    // e6:SetLabel(5)
    effect::set_label(d, e6, &[5]);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.confilter(c)`
fn confilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YANG_ZING) and c:IsMonster()
    card::is_set_card(d, c, &[SET_YANG_ZING as u16]) && card::is_monster(d, c)
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, e: EffectId) -> bool {
    // local g=Duel.GetMatchingGroup(s.confilter,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&confilter), p, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetClassCount(Card.GetAttribute)>=e:GetLabel()
    let n = group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) as i64;
    n >= effect::get_label(d, e)
}
/// `s.atktg(e,c)`: `return c:IsSetCard(SET_YANG_ZING)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_YANG_ZING as u16])
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_YANG_ZING) and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and not c:IsReason(REASON_REPLACE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_YANG_ZING as u16])
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
/// `s.repval(e,c)`: `return s.repfilter(c,e:GetHandlerPlayer())`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p) as i64
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_EFFECT, None, None);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
