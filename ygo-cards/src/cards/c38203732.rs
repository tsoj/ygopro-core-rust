//! Pendulum Paradox
use super::prelude::*;

pub const ID: u32 = 38203732;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    //   and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,c,c:GetLeftScale(),c:GetCode())
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_able_to_hand(d, c, None)
        && {
            let sc = card::get_left_scale(d, c);
            let cd = card::get_code(d, c)[0];
            duel::is_existing_matching_card(d, Some(&|d, cc| filter2(d, cc, sc, cd)), tp, LOCATION_EXTRA, 0, 1, c.into())
        }
}
/// `s.filter2(c,sc,cd)`
fn filter2(d: &mut Duel, c: CardId, sc: u32, cd: u32) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    //   and c:GetLeftScale()==sc and not c:IsCode(cd)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_able_to_hand(d, c, None)
        && card::get_left_scale(d, c) == sc
        && !card::is_code(d, c, &[cd])
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local tc1=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_EXTRA,0,1,1,nil,tp):GetFirst()
    let tp = a.tp;
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc1 = group::get_first(d, &g1);
    // if not tc1 then return end
    let Some(tc1) = tc1 else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local tc2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,tc1,tc1:GetLeftScale(),tc1:GetCode()):GetFirst()
    let sc = card::get_left_scale(d, tc1);
    let cd = card::get_code(d, tc1)[0];
    let g2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, sc, cd)), tp, LOCATION_EXTRA, 0, 1, 1, false, tc1.into()).nil();
    let tc2 = group::get_first(d, &g2);
    // Duel.SendtoHand(Group.FromCards(tc1,tc2),nil,REASON_EFFECT)
    let g = from_cards(tc1, tc2);
    duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(1-tp,Group.FromCards(tc1,tc2))
    duel::confirm_cards(d, 1 - tp, from_cards(tc1, tc2));
}
/// `Group.FromCards(a, b)`: `b` may be nil, which is left out
fn from_cards(a: CardId, b: Option<CardId>) -> CardSet {
    let mut g = CardSet::from_card(a);
    if let Some(b) = b {
        group::add_card(&mut g, &CardSet::from_card(b));
    }
    g
}
