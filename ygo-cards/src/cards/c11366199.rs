//! Dark Simorgh
use super::prelude::*;

pub const ID: u32 = 11366199;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add WIND attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(ATTRIBUTE_WIND)
    effect::set_value(d, e1, ATTRIBUTE_WIND as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCost(s.spcost1)
    effect::set_cost(d, e2, Some(cost_target(spcost1)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon itself from the grave
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCost(s.spcost2)
    effect::set_cost(d, e3, Some(cost_target(spcost2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot set
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e4, EFFECT_CANNOT_MSET);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // e4:SetTarget(aux.TRUE)
    effect::set_target(d, e4, Some(aux_true()));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e5, EFFECT_CANNOT_SSET);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e4:Clone()
    let e6 = effect::clone(d, e4);
    // e6:SetCode(EFFECT_CANNOT_TURN_SET)
    effect::set_code(d, e6, EFFECT_CANNOT_TURN_SET);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e4:Clone()
    let e7 = effect::clone(d, e4);
    // e7:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e7, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e7:SetTarget(s.sumlimit)
    effect::set_target(d, e7, Some(lua_fn(sumlimit)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(_d: &mut Duel, a: Args) -> bool {
    // return (sumpos&POS_FACEDOWN)>0
    (a.int(4) & POS_FACEDOWN as i64) > 0
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.atchk1,1,nil,sg)
    let (ok, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    (ok && group::is_exists(d, sg, &|d, x| atchk1(d, x, sg), 1, NO_EX), false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and sg:FilterCount(Card.IsAttribute,c,ATTRIBUTE_DARK)==1
    card::is_attribute(d, c, ATTRIBUTE_WIND) && group::filter_count(d, sg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), c.into()) == 1
}
/// `s.spfilter1(c,att)`
fn spfilter1(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcost1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rg1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_WIND)
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter1(d, x, ATTRIBUTE_WIND)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_DARK)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter1(d, x, ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=rg1:Clone()
    let mut rg = rg1.clone();
    // rg:Merge(rg2)
    group::add_card(&mut rg, &rg2);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-2 and #rg1>0 and #rg2>0 and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return ft > -2
            && !rg1.is_empty()
            && !rg2.is_empty()
            && matches!(
                aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,false)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    }
    true
}
/// `s.spfilter2(c,att)`
fn spfilter2(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsAbleToRemoveAsCost()
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.spcost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rg1=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_HAND,0,nil,ATTRIBUTE_WIND)
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter2(d, x, ATTRIBUTE_WIND)), tp, LOCATION_HAND, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_HAND,0,nil,ATTRIBUTE_DARK)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter2(d, x, ATTRIBUTE_DARK)), tp, LOCATION_HAND, 0, NO_EX);
    // local rg=rg1:Clone()
    let mut rg = rg1.clone();
    // rg:Merge(rg2)
    group::add_card(&mut rg, &rg2);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-2 and #rg1>0 and #rg2>0 and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return ft > -2
            && !rg1.is_empty()
            && !rg2.is_empty()
            && matches!(
                aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,false)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    }
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
