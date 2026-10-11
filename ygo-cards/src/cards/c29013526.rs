//! Storming Wynn
use super::prelude::*;

pub const ID: u32 = 29013526;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,ft,tp)`
fn cfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and (ft>0 or (c:GetSequence()<5 and c:IsControler(tp))) and (c:IsFaceup() or c:IsControler(tp))
    card::is_attribute(d, c, ATTRIBUTE_WIND)
        && (ft > 0 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)))
        && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,e:GetHandler(),ft,tp)
        let h = effect::get_handler(d, e).nil();
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, ft, tp)), 1, 1, false, None, h.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,e:GetHandler(),ft,tp)
    let h = effect::get_handler(d, e).nil();
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, ft, tp)), 1, 1, false, None, h.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
        //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_LEAVE_FIELD)
        effect::set_code(d, e1, EVENT_LEAVE_FIELD);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // g:GetFirst():RegisterEffect(e1)
        let tc = group::get_first(d, &g).nil();
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // if eg:IsExists(Card.IsCode,1,nil,id) then
    if group::is_exists(d, &eg, &|d, c| card::is_code(d, c, &[ID]), 1, NO_EX) {
        // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_EFFECT, None, None);
    }
}
