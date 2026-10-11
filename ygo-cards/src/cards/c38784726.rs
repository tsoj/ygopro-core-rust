//! Rise of the Salamangreat
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 38784726;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_SALAMANGREAT}
    listed_series: &[SET_SALAMANGREAT],
    // s.listed_names={16313112}
    listed_names: &[16313112],
    // s.fit_monster={16313112}
    fit_monster: &[16313112],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // Ritual.AddProcGreater{handler=c,filter=s.ritualfil,extrafil=s.extrafil,extraop=s.extraop,extratg=s.extratg}
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| ritualfil(d, x))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            extraop: Some(Arc::new(|d: &mut Duel, mg: &CardSet, a: &EffArgs, _tc: CardId| extraop(d, mg, a))),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // Special Summon 1 "Salamangreat Emerald Eagle from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.ritualfil(c)
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SALAMANGREAT) and c:IsRitualMonster()
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_ritual_monster(d, c)
}
// function s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsSetCard(SET_SALAMANGREAT) and c:IsAbleToDeck()
    card::has_level(d, c) && card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_able_to_deck(d, c)
}
// function s.ckfilter(c,e,tp)
fn ckfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsLinkMonster()
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_link_monster(d, c)
}
// function s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // if Duel.IsExistingMatchingCard(s.ckfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) then
    if duel::is_existing_matching_card(d, Some(&|d, c| ckfilter(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_GRAVE,0,nil)
        return Some(duel::get_matching_group(d, Some(&|d, c| mfilter(d, c)), a.tp, LOCATION_GRAVE, 0, NO_EX));
    }
    // return Group.CreateGroup()
    Some(CardSet::new())
}
// function s.extraop(mg,e,tp,eg,ep,ev,re,r,rp)
fn extraop(d: &mut Duel, mg: &CardSet, _a: &EffArgs) {
    // local mat2=mg:Filter(Card.IsLocation,nil,LOCATION_GRAVE):Filter(s.mfilter,nil)
    let mut mg = mg.clone();
    let grave = group::filter(d, &mg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    let mat2 = group::filter(d, &grave, &|d, c| mfilter(d, c), NO_EX);
    // mg:Sub(mat2)
    group::remove_card(&mut mg, &mat2);
    // Duel.ReleaseRitualMaterial(mg)
    duel::release_ritual_material(d, &mg, false);
    // Duel.SendtoDeck(mat2,nil,SEQ_DECKSHUFFLE,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)
    duel::send_to_deck(d, mat2, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None);
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsPreviousControler(tp) and c:IsReason(REASON_EFFECT)
    a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp) && card::is_reason(d, c, REASON_EFFECT)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(16313112) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[16313112]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #sg>0 then Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP) end
    if !sg.is_empty() {
        duel::special_summon(d, sg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
