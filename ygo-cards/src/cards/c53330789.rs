//! Vanquish Soul Trinity Burst
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 53330789;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VANQUISH_SOUL)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,c:GetOriginalAttribute())
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])) {
        return false;
    }
    let att = card::get_original_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, att)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,att)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:GetOriginalAttribute()~=att and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && card::get_original_attribute(d, c) != att
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let _ = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.thfilter(c,p)`
fn thfilter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return c:IsAbleToHand() and c:IsControler(p)
    card::is_able_to_hand(d, c, None) && card::is_controler(d, c, p)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() or ft<=0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) || ft <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp,tc:GetOriginalAttribute())
    let att = card::get_original_attribute(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, tp, att)), tp, LOCATION_HAND, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=math.min(2,ft)
    let mut ct = ft.min(2);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ct,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let dn = aux::dncheck();
    let rs: aux::Rescon = &dn;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ct as usize), Some(rs), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // local og=Group.CreateGroup()
    let mut og = CardSet::new();
    // for sc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
            // og:AddCard(sc)
            group::add_card(&mut og, &CardSet::from_card(sc));
        }
    }
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // if #og>0 then
    if !og.is_empty() {
        // aux.DelayedOperation(og,PHASE_END,id,e,tp,function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end,nil,0)
        aux::delayed_operation(d, og.clone(), PHASE_END, ID, a.e, tp, Some(std::sync::Arc::new(delayed_to_hand)), None, Some(0), None, None, None);
        // local thg=Group.CreateGroup()
        let mut thg = CardSet::new();
        // for oc in (og+tc):Iter() do
        let mut og_tc = og.clone();
        group::add_card(&mut og_tc, &CardSet::from_card(tc));
        for oc in group::iter(d, &og_tc) {
            // thg:Merge(oc:GetColumnGroup():Filter(s.thfilter,nil,1-tp))
            let cg = card::get_column_group(d, oc, 0, 0);
            let f = group::filter(d, &cg, &|d, x| thfilter(d, x, 1 - tp), NO_EX);
            group::add_card(&mut thg, &f);
        }
        // if #thg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !thg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(thg,nil,REASON_EFFECT)
            duel::send_to_hand(d, thg, None, REASON_EFFECT, None);
        }
    }
}
/// `function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end` (the `aux.DelayedOperation` callback)
fn delayed_to_hand(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
    true
}
