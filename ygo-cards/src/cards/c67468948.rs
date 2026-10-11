//! Layard the Liberator
use super::prelude::*;

pub const ID: u32 = 67468948;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // tohand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FAIRY) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FAIRY) && card::is_able_to_hand(d, c, None)
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    // if not re:IsHasType(EFFECT_TYPE_ACTIVATE) or not re:IsActiveType(TYPE_COUNTER) then return end
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || !effect::is_active_type(d, re, TYPE_COUNTER) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_REMOVED, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sg=g:Select(tp,2,2,nil)
    let sg = group::select(d, &g, tp, 2, 2, false, NO_EX).nil();
    // Duel.SendtoHand(sg,nil,REASON_EFFECT)
    duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg);
}
