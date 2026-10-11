//! Sinful Spoils of the White Forest
use super::prelude::*;

pub const ID: u32 = 3496543;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_FIEND|RACE_ILLUSION|RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Set this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsReason(REASON_COST) and re:IsActivated() and re:IsMonsterEffect() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_FIEND|RACE_ILLUSION|RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil) end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND | RACE_ILLUSION | RACE_SPELLCASTER)),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsReason(REASON_COST) and re:IsActivated() and re:IsMonsterEffect() end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_reason(d, h, REASON_COST) {
        return false;
    }
    let re = a.re.nil();
    effect::is_activated(d, re) && effect::is_monster_effect(d, re)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FIEND|RACE_ILLUSION|RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FIEND | RACE_ILLUSION | RACE_SPELLCASTER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `Fusion.SummonEffTG()(e,tp,eg,ep,ev,re,r,rp,chk)`
fn fus_tg(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let mut call = a.clone();
    call.chk = Some(chk);
    fusion::summon_eff_tg_call(d, &fusion::SummonEff::default(), &call)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local fustg=Fusion.SummonEffTG()
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=fustg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = fus_tg(d, a, 0);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // fustg(e,tp,eg,ep,ev,re,r,rp,1)
        fus_tg(d, a, 1);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == 2 {
        // Fusion.SummonEffOP()(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fusion::SummonEff::default(), a);
    }
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, tp, c, None, true);
    }
}
