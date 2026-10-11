//! Lyrilusc - Sapphire Swallow
use super::prelude::*;

pub const ID: u32 = 60954556;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LYRILUSC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself and a Level 1 Winged Beast monster from the hand
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
    // Provide an effect to a WIND Xyz monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetProperty(EFFECT_FLAG_EVENT_PLAYER)
    effect::set_property(d, e2, EFFECT_FLAG_EVENT_PLAYER, 0);
    // e2:SetCondition(s.efcon)
    effect::set_condition(d, e2, Some(condition(efcon)));
    // e2:SetOperation(s.efop)
    effect::set_operation(d, e2, Some(operation(efop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_WINGEDBEAST),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_WINGEDBEAST))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:GetLevel()==1 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WINGEDBEAST)
        && card::get_level(d, c) == 1
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,c,e,tp) end
    if a.chk == Some(0) {
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 1 {
            return false;
        }
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return false;
        }
        if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
            return false;
        }
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,c,e,tp)
    let mut g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // if #g>0 then
    if !g.is_empty() {
        // g:AddCard(c)
        group::add_card(&mut g, &CardSet::from_card(c));
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.efcon(e,tp,eg,ep,ev,re,r,rp)`
fn efcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return r==REASON_XYZ and c:GetReasonCard():IsAttribute(ATTRIBUTE_WIND)
    if a.r != REASON_XYZ {
        return false;
    }
    let rc = card::get_reason_card(d, c).nil();
    card::is_attribute(d, rc, ATTRIBUTE_WIND)
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.xyzcon)
    effect::set_condition(d, e1, Some(condition(xyzcon)));
    // e1:SetTarget(s.xyztg)
    effect::set_target(d, e1, Some(cost_target(xyztg)));
    // e1:SetOperation(s.xyzop)
    effect::set_operation(d, e1, Some(operation(xyzop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        // e2:SetValue(TYPE_EFFECT)
        effect::set_value(d, e2, TYPE_EFFECT as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e2,true)
        card::register_effect(d, rc, e2, true);
    }
}
/// `s.xyzcon(e,tp,eg,ep,ev,re,r,rp)`
fn xyzcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LYRILUSC) and c:IsMonster()
    card::is_set_card(d, c, &[SET_LYRILUSC as u16]) && card::is_monster(d, c)
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.xyzfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && xyzfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xyzfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // local g=Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&xyzfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) && card::is_relate_to_effect(d, tc.nil(), e) {
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, tc.nil(), false);
    }
}
