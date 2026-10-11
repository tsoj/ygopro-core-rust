//! Reptilianne Coatl
//! レプティレス・コアトル
use super::prelude::*;

pub const ID: u32 = 89594399;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_REPTILIANNE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon self from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Treat as non-tuner for the Synchro Summmon of a Reptile monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_NONTUNER)
    effect::set_code(d, e2, EFFECT_NONTUNER);
    // e2:SetValue(function(e,sc) return sc:IsRace(RACE_REPTILE) end)
    effect::set_value_fn(d, e2, effect_card_fn(nontuner_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,sc) return sc:IsRace(RACE_REPTILE) end`
fn nontuner_value(d: &mut Duel, _e: EffectId, sc: CardId) -> bool {
    card::is_race(d, sc, RACE_REPTILE)
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_REPTILE)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_REPTILE)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_REPTILIANNE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_REPTILIANNE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)<1 then return end
    if !card::is_relate_to_effect(d, c, e)
        || duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) < 1
    {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local og=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttack,0),tp,0,LOCATION_MZONE,0,nil)
    let og = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc) && card::is_attack(d, cc, &[0])), tp, 0, LOCATION_MZONE, NO_EX);
    // if ft==0 or #og==0 then return end
    if ft == 0 || og.is_empty() {
        return;
    }
    // local ct=math.min(#og,ft)
    let mut ct = std::cmp::min(og.len() as i32, ft);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if ct>0 and #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if ct > 0 && !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local ssg=sg:Select(tp,1,ct,nil)
        let ssg = group::select(d, &sg, tp, 1, ct as u16, false, NO_EX).nil();
        // if #ssg>0 then Duel.SpecialSummon(ssg,0,tp,tp,false,false,POS_FACEUP) end
        if !ssg.is_empty() {
            duel::special_summon(d, ssg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
