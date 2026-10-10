//! Question
use super::prelude::*;

pub const ID: u32 = 38723936;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
    // return c:IsMonster() and c:IsAbleToRemove()
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local last=g:GetFirst()
    let mut last = group::get_first(d, &g).nil();
    // local tc=g:GetNext() (see below)
    // (GetNext has no effect in the port: groups are values and the loop below iterates them.)
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if tc:GetSequence()<last:GetSequence() then last=tc end
        if card::get_sequence(d, tc) < card::get_sequence(d, last) {
            last = tc;
        }
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CODE as u64);
    // local ac=Duel.AnnounceCard(1-tp,TYPE_MONSTER)
    let ac = duel::announce_card(d, 1 - a.tp, &[TYPE_MONSTER as u64]);
    // if ac~=last:GetCode() then
    let last_code = card::get_code(d, last);
    if ac as u32 != last_code[0] {
        // Duel.SpecialSummon(last,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, last, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else {
        // Duel.Remove(last,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, last, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
