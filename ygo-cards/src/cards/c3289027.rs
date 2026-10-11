//! Parl, Hermit of Dark World
//! 暗黒界の隠者 パアル
use super::prelude::*;

pub const ID: u32 = 3289027;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DARK_WORLD], ..ScriptInfo::DEFAULT };

/// `LOCATION_HAND_GRAVE_REMOVED` (a local of the Lua script)
const LOCATION_HAND_GRAVE_REMOVED: u32 = LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Dark World" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
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
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if rp==1-tp and c:IsPreviousControler(tp) then
    if a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    // return c:IsPreviousLocation(LOCATION_HAND) and r&(REASON_DISCARD+REASON_EFFECT)==REASON_DISCARD+REASON_EFFECT
    card::is_previous_location(d, c, LOCATION_HAND) && (a.r & (REASON_DISCARD + REASON_EFFECT)) == REASON_DISCARD + REASON_EFFECT
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DARK_WORLD) and not c:IsCode(id) and
    //     ((Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) or
    //     (Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)))
    card::is_set_card(d, c, &[SET_DARK_WORLD as u16])
        && !card::is_code(d, c, &[ID])
        && ((duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
            || (duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter1(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter1,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter1(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter1,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter1(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND_GRAVE_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND_GRAVE_REMOVED as i32);
    true
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FIEND) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and
    //     ((Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) or
    //     (Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)))
    card::is_race(d, c, RACE_FIEND)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && ((duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
            || (duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)))
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None);
    // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    let b2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let sel_op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // local target_player=op==1 and tp or 1-tp
    let target_player = if sel_op == Some(1) { tp } else { 1 - tp };
    // if Duel.SpecialSummon(tc,0,tp,target_player,false,false,POS_FACEUP)==0 then return end
    if duel::special_summon(d, tc, 0, tp, target_player, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // Optional Summon of a Fiend monster
    // if e:GetLabel()==1 and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_HAND_GRAVE_REMOVED,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if effect::get_label(d, a.e) == 1
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, a.e, tp)), tp, LOCATION_HAND_GRAVE_REMOVED, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_HAND_GRAVE_REMOVED,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, a.e, tp)), tp, LOCATION_HAND_GRAVE_REMOVED, 0, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sel) else { return };
        // local b3=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b3 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None);
        // local b4=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
        let b4 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
        // if not (b3 or b4) then return end
        if !(b3 || b4) {
            return;
        }
        // local op2=Duel.SelectEffect(tp,{b3,aux.Stringid(id,1)},{b4,aux.Stringid(id,2)})
        let sel_op2 = duel::select_effect(d, tp, &[(b3, aux::stringid(ID, 1)), (b4, aux::stringid(ID, 2))]);
        // local target_player2=op2==1 and tp or 1-tp
        let target_player2 = if sel_op2 == Some(1) { tp } else { 1 - tp };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sc,0,tp,target_player2,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, target_player2, false, false, POS_FACEUP as u8, None);
    }
}
