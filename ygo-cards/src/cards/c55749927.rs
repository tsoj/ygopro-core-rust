//! Horn of Olifant
use super::prelude::*;

pub const ID: u32 = 55749927;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROLAND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove and destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE | TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rvmop)
    effect::set_operation(d, e1, Some(operation(rvmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate2
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_END_PHASE|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsType(TYPE_EQUIP) and c:IsAbleToRemove()
    card::is_type(d, c, TYPE_SPELL) && card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.rvmop(e,tp,eg,ep,ev,re,r,rp)`
fn rvmop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g and Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local dg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let dg = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            // local tc=dg:Select(tp,1,1,nil)
            let tc = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(tc)
            duel::hint_selection(d, tc.clone(), true);
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ROLAND) and c:IsDestructable(e)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ROLAND as u16]) && card::is_destructable(d, c, Some(e))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACE_WARRIOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_race(d, c, RACE_WARRIOR as u64)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD)
    let ct = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE,0,1,nil,e)
    //     and g:CheckWithSumEqual(Card.GetLevel,9,1,ct) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && group::check_with_sum_equal(d, &g, &|d, x| card::get_level(d, x) as i64, 9, 1, ct).0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil,e)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| desfilter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g and Duel.Destroy(g,REASON_EFFECT)>0 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // if ft==0 then return end
        if ft == 0 {
            return;
        }
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // if ft>3 then ft=3 end
            if ft > 3 {
                ft = 3;
            }
            // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
            if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
                ft = 1;
            }
            // if g:CheckWithSumEqual(Card.GetLevel,9,1,ft) then
            if group::check_with_sum_equal(d, &g, &|d, x| card::get_level(d, x) as i64, 9, 1, ft).0 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:SelectWithSumEqual(tp,Card.GetLevel,9,1,ft)
                let sg = group::select_with_sum_equal(d, &g, a.tp, &|d, x| card::get_level(d, x) as i64, 9, 1, ft).nil();
                // local tc=sg:GetFirst()
                let _tc = group::get_first(d, &sg);
                // for tc in aux.Next(sg) do
                for tc in group::iter(d, &sg) {
                    // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
                    if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
                        // local e1=Effect.CreateEffect(e:GetHandler())
                        let h = effect::get_handler(d, a.e).nil();
                        let e1 = effect::create_effect(d, h);
                        // e1:SetType(EFFECT_TYPE_SINGLE)
                        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                        // e1:SetCode(EFFECT_DISABLE)
                        effect::set_code(d, e1, EFFECT_DISABLE);
                        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                        // tc:RegisterEffect(e1,true)
                        card::register_effect(d, tc, e1, true);
                        // local e2=Effect.CreateEffect(e:GetHandler())
                        let h = effect::get_handler(d, a.e).nil();
                        let e2 = effect::create_effect(d, h);
                        // e2:SetType(EFFECT_TYPE_SINGLE)
                        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                        // e2:SetCode(EFFECT_DISABLE_EFFECT)
                        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                        // tc:RegisterEffect(e2,true)
                        card::register_effect(d, tc, e2, true);
                    }
                }
                // Duel.SpecialSummonComplete()
                duel::special_summon_complete(d);
            }
        }
    }
    // local e0=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e0 = effect::create_effect(d, h);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e0:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e0, aux::stringid(ID, 3));
    // e0:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e0, EFFECT_CANNOT_SPECIAL_SUMMON);
    // if Duel.IsTurnPlayer(tp) then
    //     e0:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
    // else
    //     e0:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN)
    // end
    if duel::is_turn_player(d, a.tp) {
        effect::set_reset(d, e0, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    } else {
        effect::set_reset(d, e0, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
    }
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetTarget(s.splimit)
    effect::set_target(d, e0, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsRace(RACE_WARRIOR)
    !card::is_race(d, c, RACE_WARRIOR as u64)
}
