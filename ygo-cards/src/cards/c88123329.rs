//! 精霊獣 ラムペンタ / Spiritual Beast Rampengu
use super::prelude::*;

pub const ID: u32 = 88123329;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RITUAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // tohand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,rac)`
fn tgfilter(d: &mut Duel, c: CardId, rac: u64) -> bool {
    // return c:IsSetCard(SET_RITUAL_BEAST) and c:IsRace(rac) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_RITUAL_BEAST as u16]) && card::is_race(d, c, rac) && card::is_able_to_grave(d, c, None)
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RITUAL_BEAST) and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,c:GetRace())
    card::is_set_card(d, c, &[SET_RITUAL_BEAST as u16]) && card::is_able_to_remove(d, c, None, None, None) && {
        let rac = card::get_race(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, rac)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.Remove(g,POS_FACEUP,REASON_EFFECT)~=0 then
    if !g.is_empty() && duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil,g:GetFirst():GetRace())
        let first = group::get_first(d, &g).nil();
        let rac = card::get_race(d, first);
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| tgfilter(d, c, rac)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
