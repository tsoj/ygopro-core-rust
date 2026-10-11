//! Stardust Shimmer
use super::prelude::*;

pub const ID: u32 = 89181369;

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
/// `s.spfilter(c,e,tp,rg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rg: &CardSet) -> bool {
    // if not c:IsType(TYPE_SYNCHRO) or not c:IsRace(RACE_DRAGON) or not c:IsCanBeSpecialSummoned(e,0,tp,false,false) then return false end
    if !card::is_type(d, c, TYPE_SYNCHRO)
        || !card::is_race(d, c, RACE_DRAGON)
        || !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
    {
        return false;
    }
    // if rg:IsContains(c) then
    let result;
    if group::is_contains(rg, c) {
        // rg:RemoveCard(c)
        let mut rg2 = rg.clone();
        group::remove_card(&mut rg2, &CardSet::from_card(c));
        // result=rg:CheckWithSumEqual(Card.GetLevel,c:GetLevel(),1,99)
        let lv = card::get_level(d, c);
        result = group::check_with_sum_equal(d, &rg2, &|d, x| card::get_level(d, x) as i64, lv, 1, 99).0;
        // rg:AddCard(c)
    } else {
        // result=rg:CheckWithSumEqual(Card.GetLevel,c:GetLevel(),1,99)
        let lv = card::get_level(d, c);
        result = group::check_with_sum_equal(d, rg, &|d, x| card::get_level(d, x) as i64, lv, 1, 99).0;
    }
    // return result
    result
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::has_level(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_SPIRIT_ELIMINATION).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        {
            return false;
        }
        // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
        let rg = duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
        // return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,rg)
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, &rg)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,rg)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, &rg)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e)
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None)
    {
        return;
    }
    // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let mut rg = duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // rg:RemoveCard(tc)
    group::remove_card(&mut rg, &CardSet::from_card(tc));
    // if rg:CheckWithSumEqual(Card.GetLevel,tc:GetLevel(),1,99) then
    let tc_lv = card::get_level(d, tc);
    if group::check_with_sum_equal(d, &rg, &|d, x| card::get_level(d, x) as i64, tc_lv, 1, 99).0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local rm=rg:SelectWithSumEqual(tp,Card.GetLevel,tc:GetLevel(),1,99)
        let rm = group::select_with_sum_equal(d, &rg, a.tp, &|d, x| card::get_level(d, x) as i64, tc_lv, 1, 99).nil();
        // Duel.Remove(rm,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rm, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
