//! Dark Templar @Ignister
use super::prelude::*;

pub const ID: u32 = 97383507;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,3,3,s.lcheck)
    link::add_procedure(d, c, None, 3, Some(3), Some(group_check(lcheck)), None, None);
    // Special Summon "@Ignister" monsters from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.zptcon(nil))
    effect::set_condition(d, e1, Some(aux::zptcon(None)));
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(cost_target(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(operation(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Cyberse monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.bdocon)
    effect::set_condition(d, e2, Some(aux::bdocon()));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop2)
    effect::set_operation(d, e2, Some(operation(spop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`: `g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    group::check_different_property(d, g, &code_list)
}
fn code_list(d: &mut Duel, c: CardId) -> Vec<i64> {
    card::get_code(d, c).iter().map(|x| *x as i64).collect()
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_IGNISTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_IGNISTER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,e:GetHandler():GetLinkedZone(tp)) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let zone = card::get_linked_zone(d, h, Some(a.tp));
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=c:GetLinkedZone(tp)
    let zone = card::get_linked_zone(d, c, Some(a.tp));
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,ft,ft,nil,e,tp,zone)
    let nv = aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, zone));
    let sg = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_GRAVE, 0, ft as u16, ft as u16, false, NO_EX).nil();
    // for sc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP,zone)~=0 then
        // (SpecialSummonStep returns a boolean in Lua, which is never equal to 0: always taken)
        duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e2)
        card::register_effect(d, sc, e2, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp)
    card::is_race(d, c, RACE_CYBERSE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nv = aux::necro_valley_filter(|d, c| filter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
