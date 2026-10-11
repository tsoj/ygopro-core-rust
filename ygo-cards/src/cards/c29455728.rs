//! Twin Photon Lizard
use super::prelude::*;

pub const ID: u32 = 29455728;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PHOTON as u32],
    material_setcode: &[SET_PHOTON as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PHOTON),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_PHOTON as u16], Some(fc), st, Some(tp)))),
            2,
        )],
    );
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.mgfilter(c,e,tp,fusc,mg)`
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fusc: CardId, mg: &CardSet) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    //   and (c:GetReason()&(REASON_FUSION|REASON_MATERIAL))==(REASON_FUSION|REASON_MATERIAL) and c:GetReasonCard()==fusc
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and fusc:CheckFusionMaterial(mg,c,PLAYER_NONE+65536)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & (REASON_FUSION | REASON_MATERIAL)) == (REASON_FUSION | REASON_MATERIAL)
        && card::get_reason_card(d, c) == Some(fusc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::check_fusion_material(d, fusc, Some(mg), Some(&CardSet::from_card(c)), Some(PLAYER_NONE as u64 + 65536))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetMaterial()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_material(d, h);
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=#g
        let ct = g.len() as i64;
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
        // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
        if card::get_sequence(d, h) < 5 {
            ft += 1;
        }
        // return ct>0 and ft>=ct and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //   and e:GetHandler():IsFusionSummoned()
        //   and g:FilterCount(s.mgfilter,nil,e,tp,e:GetHandler(),g)==ct
        return ct > 0
            && ft >= ct
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && card::is_fusion_summoned(d, h)
            && group::filter_count(d, &g, &|d, c| mgfilter(d, c, a.e, a.tp, h, &g), NO_EX) as i64 == ct;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local g=e:GetHandler():GetMaterial()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_material(d, h);
    // local ct=#g
    let ct = g.len() as i64;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>=ct
    //   and g:FilterCount(s.mgfilter,nil,e,tp,e:GetHandler(),g)==ct then
    let lc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    if lc >= ct && group::filter_count(d, &g, &|d, c| mgfilter(d, c, a.e, a.tp, h, &g), NO_EX) as i64 == ct {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
