//! De-Fusion
use super::prelude::*;

pub const ID: u32 = 95286165;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:IsAbleToExtra()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION) && card::is_able_to_extra(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.mgfilter(c,e,tp,fusc,mg)`
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fusc: CardId, mg: &CardSet) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    //     and (c:GetReason()&(REASON_FUSION|REASON_MATERIAL))==(REASON_FUSION|REASON_MATERIAL) and c:GetReasonCard()==fusc
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and fusc:CheckFusionMaterial(mg,c,PLAYER_NONE|FUSPROC_NOTFUSION)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & (REASON_FUSION | REASON_MATERIAL)) == (REASON_FUSION | REASON_MATERIAL)
        && card::get_reason_card(d, c) == Some(fusc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::check_fusion_material(d, fusc, Some(mg), Some(&CardSet::from_card(c)), Some((PLAYER_NONE | FUSPROC_NOTFUSION) as u64))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // local mg=tc:GetMaterial()
    let mg = card::get_material(d, tc);
    // local ct=#mg
    let ct = mg.len();
    // local sumtype=tc:GetSummonType()
    let sumtype = card::get_summon_type(d, tc);
    // if Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)~=0 and (sumtype&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
    //     and ct>0 and ct<=Duel.GetLocationCount(tp,LOCATION_MZONE)
    //     and mg:FilterCount(aux.NecroValleyFilter(s.mgfilter),nil,e,tp,tc,mg)==ct
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0
        && (sumtype & SUMMON_TYPE_FUSION) == SUMMON_TYPE_FUSION
        && ct > 0
        && (ct as i32) <= duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0
        && group::filter_count(d, &mg, &aux::necro_valley_filter(|d, c| mgfilter(d, c, a.e, a.tp, tc, &mg)), NO_EX) == ct
        && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(mg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, mg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
