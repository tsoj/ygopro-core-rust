//! Instant Contact
use super::prelude::*;

pub const ID: u32 = 16169772;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NEOS], listed_series: &[SET_ELEMENTAL_HERO, SET_NEO_SPACIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
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
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(7) and c:IsSetCard({SET_ELEMENTAL_HERO,SET_NEO_SPACIAN})
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_level_below(d, c, 7)
        && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16, SET_NEO_SPACIAN as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc then return end
    let Some(tc) = tc else { return };
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_NEOS),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil) then
    if duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_NEOS]))),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    ) {
        // Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    // elseif Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
    } else if duel::special_summon_step(d, tc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --Cannot attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // --Effects are negated
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e2, EFFECT_DISABLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        // e3:SetValue(RESET_TURN_SET)
        effect::set_value(d, e3, RESET_TURN_SET as i64);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // --Return it to the Extra Deck during the End Phase
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e4:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
        // e4:SetCountLimit(1)
        effect::set_count_limit(d, e4, 1, 0, 0, 0);
        // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e4:SetLabelObject(tc)
        effect::set_label_object(d, e4, Value::Card(tc));
        // e4:SetCondition(s.tdcon)
        effect::set_condition(d, e4, Some(condition(tdcon)));
        // e4:SetOperation(s.tdop)
        effect::set_operation(d, e4, Some(operation(tdop)));
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, a.tp);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if not tc or tc:GetFlagEffect(id)==0 then
    let flagged = tc.is_some_and(|tc| card::get_flag_effect(d, tc, ID) != 0);
    if !flagged {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // return true
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc then
    if let Some(tc) = tc {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
