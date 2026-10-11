//! Ninjitsu Art of Super-Transformation
//! 忍法 超変化の術
use super::prelude::*;

pub const ID: u32 = 90200789;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter1(c,tp,slv,ft)`
fn filter1(d: &mut Duel, c: CardId, tp: u8, slv: i64, ft: i64) -> bool {
    // local lv1=c:GetLevel()
    let lv1 = card::get_level(d, c) as i64;
    // return c:IsFaceup() and c:IsSetCard(SET_NINJA) and lv1>0 and (ft>0 or c:GetSequence()<5)
    //     and Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil,lv1,slv)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_NINJA as u16])
        && lv1 > 0
        && (ft > 0 || card::get_sequence(d, c) < 5)
        && duel::is_existing_target(d, Some(&|d, x| filter2(d, x, lv1, slv)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.filter2(c,lv1,slv)`
fn filter2(d: &mut Duel, c: CardId, lv1: i64, slv: i64) -> bool {
    // local lv2=c:GetLevel()
    let lv2 = card::get_level(d, c) as i64;
    // return c:IsFaceup() and lv2>0 and lv1+lv2>=slv
    card::is_faceup(d, c) && lv2 > 0 && lv1 + lv2 >= slv
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: Option<i64>) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_DINOSAUR|RACE_SEASERPENT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (not lv or c:IsLevelBelow(lv))
    card::is_race(d, c, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && lv.is_none_or(|lv| card::is_level_below(d, c, lv as u32))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, None)), tp, LOCATION_DECK, 0, NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // if chk==0 then
    if a.chk == Some(0) {
        // if #sg==0 then return false end
        if sg.is_empty() {
            return false;
        }
        // local mg,mlv=sg:GetMinGroup(Card.GetLevel)
        let (_mg, mlv) = group::get_min_group(d, &sg, &|d, c| card::get_level(d, c) as i64).nil();
        // return ft>-1 and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp,mlv,ft)
        return ft > -1 && duel::is_existing_target(d, Some(&|d, c| filter1(d, c, tp, mlv, ft)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local mg,mlv=sg:GetMinGroup(Card.GetLevel)
    let (_mg, mlv) = group::get_min_group(d, &sg, &|d, c| card::get_level(d, c) as i64).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,tp,mlv,ft)
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, tp, mlv, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil,g1:GetFirst():GetLevel(),mlv)
    let first = group::get_first(d, &g1).nil();
    let lv1 = card::get_level(d, first) as i64;
    let g2 = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, lv1, mlv)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    let mut g1 = g1;
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g1), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tg=g:Filter(Card.IsRelateToEffect,nil,e)
    let tg = group::filter(d, &g, &|d, x| card::is_relate_to_effect(d, x, e), NO_EX);
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // Duel.SendtoGrave(tg,REASON_EFFECT)
    duel::send_to_grave(d, tg.clone(), REASON_EFFECT, None, None);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=tg:GetFirst()
    let order = group::iter(d, &tg);
    let mut lv: i64 = 0;
    let tc = order[0];
    // if tc:IsLocation(LOCATION_GRAVE) then lv=lv+tc:GetLevel() end
    if card::is_location(d, tc, LOCATION_GRAVE) {
        lv += card::get_level(d, tc) as i64;
    }
    // tc=tg:GetNext()
    // if tc and tc:IsLocation(LOCATION_GRAVE) then lv=lv+tc:GetLevel() end
    if let Some(&tc) = order.get(1) {
        if card::is_location(d, tc, LOCATION_GRAVE) {
            lv += card::get_level(d, tc) as i64;
        }
    }
    // if lv==0 then return end
    if lv == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,lv)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, Some(lv))), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // c:SetCardTarget(tc)
            card::set_card_target(d, c, tc);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
