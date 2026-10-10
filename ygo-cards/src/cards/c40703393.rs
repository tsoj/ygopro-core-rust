//! The Puppet Magic of Dark Ruler
use super::prelude::*;

pub const ID: u32 = 40703393;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp,ft,rg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32, rg: &CardSet) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsRace(RACE_FIEND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and rg:CheckWithSumEqual(Card.GetLevel,lv,ft,99)
    lv > 0
        && card::is_race(d, c, RACE_FIEND)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::check_with_sum_equal(d, rg, &|d, x| card::get_level(d, x) as i64, lv, ft, 99).0
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetLevel()>0 and c:IsAbleToRemove()
    card::get_level(d, c) > 0 && card::is_able_to_remove(d, c, None, None, None)
}
/// `local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)` then `if ft<=0 then ft=-ft+1 else ft=1 end`
fn free_count(d: &mut Duel, tp: u8) -> i32 {
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft <= 0 { -ft + 1 } else { 1 }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ft=...
    let ft = free_count(d, a.tp);
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,0,nil)
        let rg = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,ft,rg)
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp, ft, &rg)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,ft,rg)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp, ft, &rg)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local ft=...
    let ft = free_count(d, tp);
    // if not tc:IsRelateToEffect(e) or not tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then return end
    if !card::is_relate_to_effect(d, tc, a.e) || !card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None) {
        return;
    }
    // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // if rg:CheckWithSumEqual(Card.GetLevel,lv,ft,99) then
    if group::check_with_sum_equal(d, &rg, &|d, x| card::get_level(d, x) as i64, lv, ft, 99).0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local rm=rg:SelectWithSumEqual(tp,Card.GetLevel,lv,ft,99)
        let rm = group::select_with_sum_equal(d, &rg, tp, &|d, x| card::get_level(d, x) as i64, lv, ft, 99).nil();
        // Duel.Remove(rm,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rm, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
