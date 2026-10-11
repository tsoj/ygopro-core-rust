//! Commande duel 12
use super::prelude::*;

pub const ID: u32 = 95200012;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.RockPaperScissors()
    let p = duel::rock_paper_scissors(d, true) as u8;
    // if Duel.GetLocationCount(p,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(p,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,p)
    let g = duel::select_matching_card(d, p, Some(&|d, c| spfilter(d, c, a.e, p)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,p,p,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, p, p, false, false, POS_FACEUP as u8, None);
    }
}
