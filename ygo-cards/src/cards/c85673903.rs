//! Altergeist Fijialert
//! Scripted by Eerie Code, anime version by Larry126
use super::prelude::*;

pub const ID: u32 = 85673903;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALTERGEIST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
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
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsSetCard(SET_ALTERGEIST) and c:IsType(TYPE_LINK) and c:IsLinkSummoned()
    card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_type(d, c, TYPE_LINK) && card::is_link_summoned(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_LINK) and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,c:GetLinkedZone(tp)&ZONES_MMZ)
    if !card::is_type(d, c, TYPE_LINK) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // (the Lua `chkc` here is the global nil: the chkc branch never runs)
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,eg:Filter(s.cfilter,nil,tp),e,tp) end
    if a.chk == Some(0) {
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let ex_group = group::filter(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, tp), NO_EX);
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, (&ex_group).into());
    }
    let ex_group = group::filter(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, tp), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,eg:Filter(s.cfilter,nil,tp),e,tp)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, (&ex_group).into());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local zone=tc:GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, tc, Some(tp)) & ZONES_MMZ;
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and zone>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc, a.e) && zone > 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone));
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ADD_SETCODE)
    effect::set_code(d, e1, EFFECT_ADD_SETCODE);
    // e1:SetValue(SET_ALTERGEIST)
    effect::set_value(d, e1, SET_ALTERGEIST as i64);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
