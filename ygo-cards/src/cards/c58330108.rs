//! Gigarays Gandora the Dragon of Destruction
use super::prelude::*;

pub const ID: u32 = 58330108;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GANDORA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ATK increase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // pay LP and apply an effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToGraveAsCost()
    card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,c)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, c.into());
    // return #g>=2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and aux.SelectUnselectGroup(g,e,tp,2,2,aux.ChkfMMZ(1),0)
    g.len() >= 2
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local sg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,c)
    let sg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, c.into());
    // local g=aux.SelectUnselectGroup(sg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &sg, Some(e), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_REMOVED,LOCATION_REMOVED)*300
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_REMOVED, LOCATION_REMOVED) as i64 * 300
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_GANDORA)
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_GANDORA as u16])
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and (c:IsLocation(LOCATION_SZONE) or aux.SpElimFilter(c,false,true))
    card::is_able_to_remove(d, c, None, None, None) && (card::is_location(d, c, LOCATION_SZONE) || aux::sp_elim_filter(d, c, false, true))
}
/// `#Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetCode)`
fn gc_of(d: &mut Duel, tp: u8) -> usize {
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // local gc=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetCode)
    let gc = gc_of(d, tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local b1=Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
        let b1 = duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
        // local b2=Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
        let b2 = duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
        // local b3=Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,c)
        let b3 = duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, 1, c.into());
        // return (gc==1 and b1) or (gc==2 and b2) or (gc>2 and b3)
        return (gc == 1 && b1) || (gc == 2 && b2) || (gc > 2 && b3);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local gc=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetCode)
    let gc = gc_of(d, tp);
    // if gc==1 then
    if gc == 1 {
        // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
        let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else {
        // local loc=LOCATION_ONFIELD
        let mut loc = LOCATION_ONFIELD;
        // if gc>2 then loc=LOCATION_ONFIELD|LOCATION_GRAVE end
        if gc > 2 {
            loc = LOCATION_ONFIELD | LOCATION_GRAVE;
        }
        // local g=Duel.GetMatchingGroup(s.rmfilter,tp,loc,loc,c)
        let g = duel::get_matching_group(d, Some(&rmfilter), tp, loc, loc, c.into());
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
