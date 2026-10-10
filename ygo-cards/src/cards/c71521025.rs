//! Red Blossoms from Underroot
use super::prelude::*;

pub const ID: u32 = 71521025;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.operation1)
    effect::set_operation(d, e1, Some(operation(op1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation2)
    effect::set_operation(d, e2, Some(operation(op2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,e)
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToRemove() and c:IsCanBeEffectTarget(e)
    card::is_spell_trap(d, c) && card::is_able_to_remove(d, c, None, None, None) && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsControler,1,nil,tp) and sg:IsExists(Card.IsControler,1,nil,1-tp)
    let mine = group::is_exists(d, sg, &|d, x| card::is_controler(d, x, tp), 1, NO_EX);
    (mine && group::is_exists(d, sg, &|d, x| card::is_controler(d, x, 1 - tp), 1, NO_EX), false)
}
// function s.target1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g = Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, e)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) and Duel.GetLocationCount(tp,LOCATION_MZONE) > 0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        ) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,sg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(sg), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, LOCATION_HAND as i32);
    true
}
// function s.operation1(e,tp,eg,ep,ev,re,r,rp)
fn op1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sg=Duel.GetTargetCards(e)
    let sg = duel::get_target_cards(d, a.e).nil();
    // if #sg>0 and Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsRelateToEffect(e) then
    if !sg.is_empty()
        && duel::remove(d, &sg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.filter1(c,e,tp)
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK) and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_GRAVE,0,1,c,e,tp,c:GetType()&(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK))
    if !(card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    let ty = card::get_type(d, c) & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK);
    duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, ty)), tp, LOCATION_GRAVE, 0, 1, c.into())
}
// function s.filter2(c,e,tp,type)
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ty: u32) -> bool {
    // return c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK) and not c:IsType(type) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK)
        && !card::is_type(d, c, ty)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.filter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && filter1(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>-1 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter1(d, x, e, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local sg=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp)
    let sg = duel::select_target(d, tp, Some(&|d, x| filter1(d, x, e, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,sg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, sg, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,0,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
// function s.operation2(e,tp,eg,ep,ev,re,r,rp)
fn op2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local type=tc:GetType()&(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK)
        let ty = card::get_type(d, tc) & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK);
        // if Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
        if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,type)
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| filter2(d, x, a.e, tp, ty)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
