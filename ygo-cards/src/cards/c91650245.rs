//! Sylvan Blessing
use super::prelude::*;

pub const ID: u32 = 91650245;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SYLVAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 "Sylvan" monster from hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAbleToDeck() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,c,e,tp)
    card::is_able_to_deck(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, c.into())
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SYLVAN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SYLVAN as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // local opt=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        let opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
        // Duel.SendtoDeck(g,nil,opt,REASON_EFFECT)
        duel::send_to_deck(d, g, None, opt, REASON_EFFECT, None);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,tc,e,tp)
        let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
        let sg = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, tc.into()).nil();
        // local sc=sg:GetFirst()
        let sc = group::get_first(d, &sg);
        // if sc then
        if let Some(sc) = sc {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // Place it either on top or bottom of deck during end phase
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetOperation(s.tdop)
            effect::set_operation(d, e1, Some(operation(tdop)));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // Unaffected by other card effects
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetDescription(3100)
            effect::set_description(d, e2, 3100);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e2:SetRange(LOCATION_MZONE)
            effect::set_range(d, e2, LOCATION_MZONE);
            // e2:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // e2:SetValue(s.efilter)
            effect::set_value_fn(d, e2, lua_fn(efilter));
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
        }
    }
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsType(TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    //     or Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))==0 then
    if card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ)
        || duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) == 0
    {
        // Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    } else {
        // Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
// function s.efilter(e,te)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    let te = a.e(1);
    let e = a.e(0);
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
