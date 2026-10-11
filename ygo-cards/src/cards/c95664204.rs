//! Zexal Catapult
//! ゼアル・カタパルト
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 95664204;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA, SET_ZW, SET_ZS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_ZW) or c:IsSetCard(SET_ZS)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_set_card(d, c, &[SET_ZW as u16]) || card::is_set_card(d, c, &[SET_ZS as u16]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// function s.lvfilter(c,lv)
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and c:HasLevel() and not c:IsLevel(lv)
    card::is_faceup(d, c) && card::has_level(d, c) && !card::is_level(d, c, &[lv])
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d: &mut Duel, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_UTOPIA),tp,LOCATION_MZONE,0,1,nil) then
    if group::get_count(&g) > 0
        && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d: &mut Duel, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UTOPIA as u16])),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
    {
        // local op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3),aux.Stringid(id,4))
        let op = duel::select_option(d, tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3), aux::stringid(ID, 4)]);
        // if op==2 then return end
        if op == 2 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local lv=op+4
        let lv = (op + 4) as u32;
        // local lg=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,0,nil,lv)
        let lg = duel::get_matching_group(d, Some(&|d: &mut Duel, c| lvfilter(d, c, lv)), tp, LOCATION_MZONE, 0, NO_EX);
        // for tc in aux.Next(lg) do
        for tc in group::iter(d, &lg) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_ZW) or c:IsSetCard(SET_ZS)) and c:IsMonster() and c:IsAbleToRemoveAsCost()
    (card::is_set_card(d, c, &[SET_ZW as u16]) || card::is_set_card(d, c, &[SET_ZS as u16]))
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return e:GetHandler():IsAbleToRemoveAsCost()
    //   and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return card::is_able_to_remove_as_cost(d, h, None)
            && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let mut g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, h.into()).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc:IsRelateToEffect(e) then
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
