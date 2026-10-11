//! White Sunfish
use super::prelude::*;

pub const ID: u32 = 52596406;
pub const INFO: ScriptInfo = ScriptInfo { self_tuner: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Treat this card as a Tuner
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_GRAVE) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_summon_location(d, c, LOCATION_GRAVE)
    })));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return not e:GetHandler():IsType(TYPE_TUNER) end end)
    effect::set_target(d, e2, Some(cost_target(|d, a| {
        if a.chk == Some(0) {
            let c = effect::get_handler(d, a.e).nil();
            return !card::is_type(d, c, TYPE_TUNER);
        }
        true
    })));
    // e2:SetOperation(s.tnop)
    effect::set_operation(d, e2, Some(operation(tnop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp,...)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, codes: &[u32]) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsLevelBelow(4) and c:IsCode(...) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FISH as u64)
        && card::is_level_below(d, c, 4)
        && card::is_code(d, c, codes)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `local codes=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_ONFIELD,0,nil):GetClass(Card.GetCode)`
fn codes_of(d: &mut Duel, tp: u8) -> Vec<u32> {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX);
    group::get_class(d, &g, &|d, c| card::get_code(d, c)[0] as i64).into_iter().map(|x| x as u32).collect()
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let codes = codes_of(d, tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE)
    //     and #codes>0 and s.spfilter(chkc,e,tp,table.unpack(codes)) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && !codes.is_empty() && spfilter(d, chkc, e, tp, &codes);
    }
    // if chk==0 then return #codes>0
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,table.unpack(codes))
    // end
    if a.chk == Some(0) {
        return !codes.is_empty()
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, tc| spfilter(d, tc, e, tp, &codes)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,table.unpack(codes))
    let g = duel::select_target(d, tp, Some(&|d, tc| spfilter(d, tc, e, tp, &codes)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g+c,2,0,0)
    let mut gc = g.nil();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(gc), 2, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.tnop(e,tp,eg,ep,ev,re,r,rp)`
fn tnop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
