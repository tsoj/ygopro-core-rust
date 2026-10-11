//! NEXT
use super::prelude::*;

pub const ID: u32 = 74414885;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NEOS], listed_series: &[SET_NEO_SPACIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_ONFIELD,0)==0 end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_ONFIELD,0)==0 end` (condition of e2)
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group(d, p, LOCATION_ONFIELD, 0).is_empty()
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_NEO_SPACIAN) or c:IsCode(CARD_NEOS)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) || card::is_code(d, c, &[CARD_NEOS]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, a: Args) -> bool {
    // return not c:IsOriginalType(TYPE_FUSION)
    !card::is_original_type(d, a.c(1), TYPE_FUSION)
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION)
    let c = a.c(1);
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if ft>5 then ft=5 end
    if ft > 5 {
        ft = 5;
    }
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let sg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(move |d, x| filter(d, x, e, tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(sg,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let g = aux::select_unselect_group(d, &sg, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2,true)
            card::register_effect(d, tc, e2, true);
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD);
            // e3:SetRange(LOCATION_MZONE)
            effect::set_range(d, e3, LOCATION_MZONE);
            // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e3:SetAbsoluteRange(tp,1,0)
            effect::set_absolute_range(d, e3, a.tp, 1, 0);
            // e3:SetTarget(s.splimit)
            effect::set_target(d, e3, Some(lua_fn(splimit)));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e3,true)
            card::register_effect(d, tc, e3, true);
            // --Lizard check
            // local e4=aux.createContinuousLizardCheck(c,LOCATION_MZONE,s.lizfilter)
            let e4 = aux::create_continuous_lizard_check(d, c, LOCATION_MZONE, Some(lua_fn(lizfilter)), None, None);
            // e4:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e4,true)
            card::register_effect(d, tc, e4, true);
        }
    }
}
