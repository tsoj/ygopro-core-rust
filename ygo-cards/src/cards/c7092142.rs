//! Torrential Reborn
use super::prelude::*;

pub const ID: u32 = 7092142;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_DESTROY) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and c:IsPreviousPosition(POS_FACEUP)
    //   and c:IsAttribute(ATTRIBUTE_WATER) and (c:GetPreviousAttributeOnField()&ATTRIBUTE_WATER)~=0
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_WATER) != 0
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsControler(tp) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=eg:FilterCount(s.spfilter,nil,e,tp)
        let ct = group::filter_count(d, &eg, &|d, c| spfilter(d, c, e, tp), NO_EX) as i32;
        // return ct>0 and (ct==1 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
        //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>=ct
        return ct > 0
            && (ct == 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none())
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= ct;
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // local g=eg:Filter(s.spfilter,nil,e,tp)
    let g = group::filter(d, &eg, &|d, c| spfilter(d, c, e, tp), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsControler(tp) and c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_relate_to_effect(d, c, e)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local sg=eg:Filter(s.spfilter2,nil,e,tp)
    let eg = a.eg.clone().nil();
    let sg = group::filter(d, &eg, &|d, c| spfilter2(d, c, e, tp), NO_EX);
    // if ft<#sg then return end
    if (ft as usize) < sg.len() {
        return;
    }
    // local ct=Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
    let ct = duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // Duel.Damage(1-tp,ct*500,REASON_EFFECT)
    duel::damage(d, 1 - tp, ct as i64 * 500, REASON_EFFECT, false, None);
}
