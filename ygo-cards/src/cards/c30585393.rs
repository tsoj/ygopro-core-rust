//! Puzzle Reborn
//! リボーン・パズル
use super::prelude::*;

pub const ID: u32 = 30585393;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // if chkc then return chkc==tc end
    if let Some(chkc) = a.chkc() {
        return tc == Some(chkc);
    }
    // if chk==0 then return #eg==1 and tc:IsPreviousControler(tp) and tc:IsPreviousLocation(LOCATION_MZONE)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and tc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and tc:IsReason(REASON_EFFECT)
    //     and tc:IsCanBeEffectTarget(e) and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        if eg.len() != 1 {
            return false;
        }
        let tc = tc.nil();
        return card::is_previous_controler(d, tc, a.tp)
            && card::is_previous_location(d, tc, LOCATION_MZONE)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_location(d, tc, LOCATION_GRAVE | LOCATION_REMOVED)
            && card::is_reason(d, tc, REASON_EFFECT)
            && card::is_can_be_effect_target(d, tc, Some(a.e))
            && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc.nil())), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
