//! Sacred Noble Knight of King Artorigus
use super::prelude::*;

pub const ID: u32 = 10613952;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_ARMS, SET_NOBLE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunction(Card.IsSetCard,SET_NOBLE_KNIGHT),5,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, _sc, _st, _tp| card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]))), Some(5), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(xyz_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn xyz_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c)
}
/// `s.filter(c,e,tp,ec)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_ARMS) and c:IsCanBeEffectTarget(e) and c:CheckUniqueOnField(tp) and c:CheckEquipTarget(ec)
    card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::check_unique_on_field(d, c, tp, None, None)
        && card::check_equip_target(d, c, ec)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e,tp,c)
    let g = duel::get_matching_group(d, Some(&|d, cc| filter(d, cc, e, tp, c)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == tp && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, e, tp, c);
    }
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_SZONE),3)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0.min(3);
    // if chk==0 then return ft>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, e, tp, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local eg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_EQUIP)
    let eg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_EQUIP as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,eg,#eg,0,0)
    let n = eg.len();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(eg), n as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if ft<#g then return end
    if (ft as usize) < g.len() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.Equip(tp,tc,c,true,true)
        duel::equip(d, tp, tc, c, true, true);
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // local c = e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsLevelAbove(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])
        && card::is_level_above(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::get_controler(d, chkc) == tp && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
