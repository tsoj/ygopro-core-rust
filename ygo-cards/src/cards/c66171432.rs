//! Super Koi Koi
//! 超こいこい
use super::prelude::*;

pub const ID: u32 = 66171432;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
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
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FLOWER_CARDIAN) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_FLOWER_CARDIAN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,30459350)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, 30459350).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, tp, LOCATION_DECK as i32);
    true
}
/// `s.cfilter(c,ft,tp)`
fn cfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return ft>0 or (c:GetSequence()<5 and c:IsControler(tp))
    ft > 0 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp))
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and e:GetHandler():IsAbleToRemoveAsCost() and Duel.CheckReleaseGroup(tp,s.cfilter,1,nil,ft,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return ft > -1
            && card::is_able_to_remove_as_cost(d, h, None)
            && duel::check_release_group(d, tp, Some(&|d, c| cfilter(d, c, ft, tp)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
    // local g=Duel.SelectReleaseGroup(tp,s.cfilter,1,1,nil,ft,tp)
    let g = duel::select_release_group(d, tp, Some(&|d, c| cfilter(d, c, ft, tp)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
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
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if Duel.IsPlayerAffectedByEffect(tp,30459350) then return end
    if duel::is_player_affected_by_effect(d, tp, 30459350).is_some() {
        return;
    }
    // Duel.ConfirmDecktop(tp,3)
    duel::confirm_decktop(d, tp, 3);
    // local g=Duel.GetDecktopGroup(tp,3)
    let mut g = duel::get_decktop_group(d, tp, 3);
    // local sg=g:Filter(s.filter,nil,e,tp)
    let mut sg = group::filter(d, &g, &|d, x| filter(d, x, e, tp), NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if ft > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if #sg>0 and ft>0 then
        if !sg.is_empty() && ft > 0 {
            // if #sg>ft then
            if sg.len() > ft as usize {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // sg=sg:Select(tp,ft,ft,nil)
                sg = group::select(d, &sg, tp, ft as u16, ft as u16, false, NO_EX).nil();
            }
            // g:Sub(sg)
            group::remove_card(&mut g, &sg);
            // local tc=sg:GetFirst()
            let _ = group::get_first(d, &sg);
            // for tc in aux.Next(sg) do
            for tc in group::iter(d, &sg) {
                // if Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
                if duel::special_summon_step(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
                    // if tc:HasLevel() then
                    if card::has_level(d, tc) {
                        // local e1=Effect.CreateEffect(c)
                        let e1 = effect::create_effect(d, c);
                        // e1:SetType(EFFECT_TYPE_SINGLE)
                        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                        // e1:SetCode(EFFECT_CHANGE_LEVEL)
                        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
                        // e1:SetValue(2)
                        effect::set_value(d, e1, 2);
                        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                        // tc:RegisterEffect(e1)
                        card::register_effect(d, tc, e1, false);
                    }
                    // local e2=Effect.CreateEffect(c)
                    let e2 = effect::create_effect(d, c);
                    // e2:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                    // e2:SetCode(EFFECT_DISABLE)
                    effect::set_code(d, e2, EFFECT_DISABLE);
                    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e2)
                    card::register_effect(d, tc, e2, false);
                    // local e3=Effect.CreateEffect(c)
                    let e3 = effect::create_effect(d, c);
                    // e3:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                    // e3:SetCode(EFFECT_DISABLE_EFFECT)
                    effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
                    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e3)
                    card::register_effect(d, tc, e3, false);
                }
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_REMOVED)
        let ct = group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_REMOVED), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // Duel.SetLP(tp,Duel.GetLP(tp)-ct*1000)
            let lp = duel::get_lp(d, tp);
            duel::set_lp(d, tp, lp - ct as i32 * 1000);
        }
    }
}
