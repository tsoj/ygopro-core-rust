//! Marincess Mandarin
use super::prelude::*;

pub const ID: u32 = 28174796;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon from hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MARINCESS),tp,LOCATION_MZONE,0,2,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16])), a.tp, LOCATION_MZONE, 0, 2, NO_EX)
}
// function s.spfilter(c,e,tp,sc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sc: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER) and c:IsLinkMonster()
    //     and (c:GetLinkedZone()&ZONES_MMZ)~=0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,c:GetLinkedZone()&ZONES_MMZ)
    if !(card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_link_monster(d, c)) {
        return false;
    }
    let zone = card::get_linked_zone(d, c, None) & ZONES_MMZ;
    zone != 0 && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.spfilter(chkc,e,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && spfilter(d, chkc, a.e, a.tp, c);
    }
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_MZONE,0,1,nil,e,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| spfilter(d, cc, e, tp, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,c)
    let _ = duel::select_target(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil())) {
        return;
    }
    // local zone=tc:GetLinkedZone()&ZONES_MMZ
    let zone = card::get_linked_zone(d, tc.nil(), None) & ZONES_MMZ;
    // if zone~=0 and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP,zone) then
    if zone != 0 && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone)) {
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
