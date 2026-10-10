//! Earthshaker
use super::prelude::*;

pub const ID: u32 = 60866277;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn desfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsFaceup() and c:IsAttribute(att)
    card::is_faceup(d, c) && card::is_attribute(d, c, att)
}
fn faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&faceup), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // if #g==0 then return false end
        if g.is_empty() {
            return false;
        }
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // local att=0
        let mut att: u32 = 0;
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // att=(att|tc:GetAttribute())
            att |= card::get_attribute(d, tc);
        }
        // return (att&att-1)~=0
        return (att & att.wrapping_sub(1)) != 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&faceup), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #sg==0 then return false end
    if sg.is_empty() {
        return;
    }
    // local tc=sg:GetFirst()
    let _ = group::get_first(d, &sg);
    // local att=0
    let mut att: u32 = 0;
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // att=(att|tc:GetAttribute())
        att |= card::get_attribute(d, tc);
    }
    // if (att&att-1)==0 then return end
    if (att & att.wrapping_sub(1)) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    // local att1=Duel.AnnounceAttribute(tp,2,att)
    let att1 = duel::announce_attribute(d, tp, 2, att);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATTRIBUTE as u64);
    // local att2=Duel.AnnounceAttribute(1-tp,1,att1)
    let att2 = duel::announce_attribute(d, 1 - tp, 1, att1);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,att2)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, att2)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
