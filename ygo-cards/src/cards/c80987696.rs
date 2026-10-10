//! Time Machine
use super::prelude::*;

pub const ID: u32 = 80987696;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // if chk==0 then return Duel.GetLocationCount(tc:GetPreviousControler(),LOCATION_MZONE)>0 and #eg==1
    //     and tc:IsLocation(LOCATION_GRAVE) and tc:IsReason(REASON_BATTLE)
    //     and tc:IsCanBeSpecialSummoned(e,0,tp,false,false,tc:GetPreviousPosition(),tc:GetPreviousControler()) end
    if a.chk == Some(0) {
        let tc = tc.nil();
        let prev_controler = card::get_previous_controler(d, tc);
        return duel::get_location_count(d, prev_controler, LOCATION_MZONE as u8, None, None, None).0 > 0
            && eg.len() == 1
            && card::is_location(d, tc, LOCATION_GRAVE)
            && card::is_reason(d, tc, REASON_BATTLE)
            && card::is_can_be_special_summoned(
                d,
                tc,
                a.e,
                0,
                a.tp,
                false,
                false,
                Some(card::get_previous_position(d, tc) as u8),
                Some(card::get_previous_controler(d, tc)),
                None,
            );
    }
    // tc:CreateEffectRelation(e)
    let tc = tc.nil();
    card::create_effect_relation(d, tc, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(eg), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tc:GetPreviousControler(),false,false,tc:GetPreviousPosition())
        let prev_controler = card::get_previous_controler(d, tc);
        let prev_position = card::get_previous_position(d, tc) as u8;
        duel::special_summon(d, tc, 0, a.tp, prev_controler, false, false, prev_position, None);
    }
}
