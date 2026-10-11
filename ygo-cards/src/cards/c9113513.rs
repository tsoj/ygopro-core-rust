//! Ostinato
//! オスティナート
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 9113513;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_MELODIOUS),Fusion.InHandMat,s.fextra,nil,nil,s.stage2,2,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, cc, _tp| card::is_set_card(d, cc, &[SET_MELODIOUS as u16]))),
            matfilter: Some(fusion::in_hand_mat()),
            extrafil: Some(Arc::new(fextra)),
            stage2: Some(Arc::new(stage2)),
            exactcount: Some(2),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
    (Some(g), None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.stage2(e,tc,tp,mg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk == 0 {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let tc = tc.nil();
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    // if chk==1 then
    if chk == 1 {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, tc, ID) != 0 {
        // return true
        true
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
/// `s.mgfilter(c,e,tp,fusc,mg)`
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fusc: CardId, mg: &CardSet) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    //     and (c:GetReason()&(REASON_FUSION|REASON_MATERIAL))==(REASON_FUSION|REASON_MATERIAL) and c:GetReasonCard()==fusc
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and fusc:CheckFusionMaterial(mg,c)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & (REASON_FUSION | REASON_MATERIAL)) == (REASON_FUSION | REASON_MATERIAL)
        && card::get_reason_card(d, c) == Some(fusc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::check_fusion_material(d, fusc, Some(mg), Some(&CardSet::from_card(c)), None)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // local mg=tc:GetMaterial()
    let mg = card::get_material(d, tc);
    // local sumtype=tc:GetSummonType()
    let sumtype = card::get_summon_type(d, tc);
    // if Duel.Destroy(tc,REASON_EFFECT)~=0
    if duel::destroy(d, tc, REASON_EFFECT, None, None) != 0
        // and (sumtype&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION and #mg>0
        && (sumtype & SUMMON_TYPE_FUSION) == SUMMON_TYPE_FUSION
        && !mg.is_empty()
        // and Duel.GetLocationCount(tp,LOCATION_MZONE)>=#mg
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= mg.len() as i32
        // and mg:IsExists(aux.NecroValleyFilter(s.mgfilter),#mg,nil,e,tp,tc,mg)
        && group::is_exists(d, &mg, &aux::necro_valley_filter(|d, c| mgfilter(d, c, e, tp, tc, &mg)), mg.len() as u16, NO_EX)
        // and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        // and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.SpecialSummon(mg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, mg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
