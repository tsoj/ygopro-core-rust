//! Beckoning Light
use super::prelude::*;

pub const ID: u32 = 16255442;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_TOHAND);
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
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local hd=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
        let mut hd = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        // if e:GetHandler():IsLocation(LOCATION_HAND) then hd=hd-1 end
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_HAND) {
            hd -= 1;
        }
        // return hd>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,hd,nil)
        return hd > 0 && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, hd as u32, NO_EX);
    }
    // local sg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let sg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // local tg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil)
    let tg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    let n = sg.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,sg,#sg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, Some(sg), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,#sg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let sg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, sg.clone(), REASON_EFFECT | REASON_DISCARD, None, None);
    // local ct=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE):GetCount()
    let ct = group::filter(d, &sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX).len();
    // local tg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil)
    let tg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if ct>0 and #tg>=ct then
    if ct > 0 && tg.len() >= ct {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sel=tg:Select(tp,ct,ct,nil)
        let sel = group::select(d, &tg, a.tp, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.SendtoHand(sel,nil,REASON_EFFECT)
        duel::send_to_hand(d, sel.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sel)
        duel::confirm_cards(d, 1 - a.tp, sel);
    }
}
