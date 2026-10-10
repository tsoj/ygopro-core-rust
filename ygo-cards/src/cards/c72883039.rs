//! Infinite Light
use super::prelude::*;

pub const ID: u32 = 72883039;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TIMELORD], listed_names: &[36894320], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCost(s.actcost)
    effect::set_cost(d, e1, Some(cost_target(actcost)));
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // untarget
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_TIMELORD))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TIMELORD as u16]))));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot to deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_TO_DECK)
    effect::set_code(d, e4, EFFECT_CANNOT_TO_DECK);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_TIMELORD))
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TIMELORD as u16]))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // spsummon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    // e5:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetCost(s.spcost)
    effect::set_cost(d, e5, Some(cost_target(spcost)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.acfilter(c)`
fn acfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(36894320) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_code(d, c, &[36894320]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.actcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn actcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.acfilter,tp,LOCATION_SZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&acfilter), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.acfilter,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&acfilter), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let chk0 = EffArgs { chk: Some(0), ..a.clone() };
    let chk1 = EffArgs { chk: Some(1), ..a.clone() };
    // if s.spcon(e,tp,eg,ep,ev,re,r,rp) and s.spcost(e,tp,eg,ep,ev,re,r,rp,0) and s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    //   and Duel.SelectYesNo(tp,94) then
    if spcon(d, a) && spcost(d, &chk0) && sptg(d, &chk0) && duel::select_yes_no(d, tp, 94) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetOperation(s.spop)
        effect::set_operation(d, a.e, Some(operation(spop)));
        // s.spcost(e,tp,eg,ep,ev,re,r,rp,1)
        spcost(d, &chk1);
        // s.sptg(e,tp,eg,ep,ev,re,r,rp,1)
        sptg(d, &chk1);
    }
    true
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TIMELORD) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_TIMELORD as u16]) && card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.spcheck(sg,e,tp,mg)`
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local ct1=sg:GetClassCount(Card.GetCode)
    let ct1 = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64);
    // local ct2=sg:GetClassCount(Card.GetLocation)
    let ct2 = group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64);
    // local ct3=#sg
    let ct3 = sg.len();
    // return ct1==ct3 and ct2==ct3, ct1~=ct3 or ct2~=ct3
    (ct1 == ct3 && ct2 == ct3, ct1 != ct3 || ct2 != ct3)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(sg,e,tp,1,ft,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let rg = aux::select_unselect_group(d, &sg, Some(a.e), tp, Some(1), Some(ft as usize), Some(&spcheck), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // Duel.SpecialSummon(rg,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon(d, rg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
}
