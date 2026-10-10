//! Defrag Dragon
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 58582979;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by sending 1 other monster from your hand to the GY. You can only Special Summon "Defrag Dragon" once per turn this way
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.handspcon)
    effect::set_condition(d, e1, Some(lua_fn(handspcon)));
    // e1:SetTarget(s.handsptg)
    effect::set_target(d, e1, Some(lua_fn(handsptg)));
    // e1:SetOperation(s.handspop)
    effect::set_operation(d, e1, Some(lua_fn(handspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is in the GY: You can banish 3 monsters from your GY with the same name; Special Summon this card. You can only use this effect of "Defrag Dragon" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.gyspcost)
    effect::set_cost(d, e2, Some(cost_target(gyspcost)));
    // e2:SetTarget(s.gysptg)
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    // e2:SetOperation(s.gyspop)
    effect::set_operation(d, e2, Some(operation(gyspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.handspcon(e,c)`
fn handspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsMonster,Card.IsAbleToGraveAsCost),tp,LOCATION_HAND,0,1,c)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_monster(d, x) && card::is_able_to_grave_as_cost(d, x)), tp, LOCATION_HAND, 0, 1, c.into())
}
/// `s.handsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn handsptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.card(9);
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsMonster,Card.IsAbleToGraveAsCost),tp,LOCATION_HAND,0,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_monster(d, x) && card::is_able_to_grave_as_cost(d, x)), tp, LOCATION_HAND, 0, c.map_or(NO_EX, |x| x.into()));
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.handspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn handspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_COST)
            duel::send_to_grave(d, g, REASON_COST, None, None);
        }
    }
}
/// `s.gyspcostfilter(c)`
fn gyspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:GetClassCount(Card.GetCode)==1
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0 && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1;
    (res, false)
}
/// `s.gyspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gyspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.gyspcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&gyspcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, h.into());
    if a.chk == Some(0) {
        // return #g>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
        return g.len() >= 3
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_REMOVE)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.Remove(sg,POS_FACEUP,REASON_COST)
    duel::remove(d, &sg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.gyspop(e,tp,eg,ep,ev,re,r,rp)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
