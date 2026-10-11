//! Cipher Spectrum
use super::prelude::*;

pub const ID: u32 = 98905;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CIPHER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_LEAVE_FIELD_P)
        effect::set_code(d, ge1, EVENT_LEAVE_FIELD_P);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CIPHER) and c:IsType(TYPE_XYZ) and c:IsControler(c:GetOwner()) and c:GetOverlayCount()>0
    let owner = card::get_owner(d, c);
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CIPHER as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_controler(d, c, owner)
        && card::get_overlay_count(d, c) > 0
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if not eg then return end
    let Some(eg) = a.eg.clone() else { return };
    // local sg=eg:Filter(s.cfilter,nil)
    let sg = group::filter(d, &eg, &cfilter, NO_EX);
    // if sg and #sg>0 then
    if sg.len() > 0 {
        // for tc in sg:Iter() do
        for tc in group::iter(d, &sg) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOGRAVE|RESET_LEAVE),0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOGRAVE | RESET_LEAVE)), 0, 1, 0, 0);
        }
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetFlagEffect(id)>0 and c:IsLocation(LOCATION_GRAVE) and c:IsControler(tp) and c:IsReason(REASON_DESTROY)
    //     and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    //     and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.exfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c:GetCode())
    card::get_flag_effect(d, c, ID) > 0
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_DESTROY)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && {
            let code = card::get_code(d, c)[0];
            duel::is_existing_matching_card(d, Some(&|d, x| exfilter(d, x, e, tp, code)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
// function s.exfilter(c,e,tp,code)
fn exfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_type(d, c, TYPE_XYZ)
        && card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return eg:IsContains(chkc) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return eg.contains(chkc) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetUsableMZoneCount(tp)>1 and eg:IsExists(s.spfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_usable_m_zone_count(d, tp, None) > 1
            && group::is_exists(d, &eg, &|d, x| spfilter(d, x, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=eg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter_select(d, &eg, tp, &|d, x| spfilter(d, x, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,tp,LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 or Duel.GetUsableMZoneCount(tp)<=1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 || duel::get_usable_m_zone_count(d, tp, None) <= 1 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    // (a Lua boolean is always true here)
    duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.exfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc:GetCode()):GetFirst()
    let code = card::get_code(d, tc)[0];
    let g = duel::select_matching_card(d, tp, Some(&|d, x| exfilter(d, x, a.e, tp, code)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
