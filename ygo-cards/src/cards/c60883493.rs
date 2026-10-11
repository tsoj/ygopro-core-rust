//! Vanquish Soul Snow Devil
//! ＶＳ 裏螺旋流雪風
use super::prelude::*;

pub const ID: u32 = 60883493;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal up to 3 monsters and apply effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c) return c:IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_EARTH|ATTRIBUTE_FIRE) and not c:IsPublic() end
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK | ATTRIBUTE_EARTH | ATTRIBUTE_FIRE) && !card::is_public(d, c)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local rvg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND,0,nil)
    let rvg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, NO_EX);
    // if chk==0 then return #rvg>0 end
    if a.chk == Some(0) {
        return !rvg.is_empty();
    }
    // local g=aux.SelectUnselectGroup(rvg,e,tp,1,3,aux.dpcheck(Card.GetAttribute),1,tp,HINTMSG_CONFIRM)
    let dp = aux::dpcheck(|d, c| card::get_attribute(d, c) as i64);
    let g = aux::select_unselect_group(
        d,
        &rvg,
        Some(a.e),
        a.tp,
        Some(1),
        Some(3),
        Some(&dp),
        1,
        a.tp,
        Some(HINTMSG_CONFIRM as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[g.len() as i64]);
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()~=0
        let res = effect::get_label(d, a.e) != 0;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local dam=400
    let mut dam: i64 = 400;
    // if ct>=1 then
    if ct >= 1 {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    }
    // if ct>=2 then dam=dam+600 end
    if ct >= 2 {
        dam += 600;
    }
    // if ct==3 then
    if ct == 3 {
        // dam=dam+800
        dam += 800;
        // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let dg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
        let n = dg.len() as u8;
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
// function s.spfilter(c,e,tp)
//     return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
// end
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local break_eff=false
    let mut break_eff = false;
    // if ct>=1 and Duel.Damage(1-tp,400,REASON_EFFECT)>0 then
    if ct >= 1 && duel::damage(d, 1 - a.tp, 400, REASON_EFFECT, false, None) > 0 {
        // break_eff=true
        break_eff = true;
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
            let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // if ct>=2 then
    if ct >= 2 {
        // if break_eff then Duel.BreakEffect() end
        if break_eff {
            duel::break_effect(d);
        }
        // Duel.Damage(1-tp,600,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 600, REASON_EFFECT, false, None);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // "Vanquish Soul" monsters cannot be destroyed by effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetTarget(function(e,c) return c:IsSetCard(SET_VANQUISH_SOUL) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]))));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
        // break_eff=true
        break_eff = true;
    }
    // if ct==3 then
    if ct == 3 {
        // if break_eff then Duel.BreakEffect() end
        if break_eff {
            duel::break_effect(d);
        }
        // if Duel.Damage(1-tp,800,REASON_EFFECT)>0 and Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::damage(d, 1 - a.tp, 800, REASON_EFFECT, false, None) > 0
            && duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
        {
            // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
            let dg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
