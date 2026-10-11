//! Fragrance Storm
use super::prelude::*;

pub const ID: u32 = 69584564;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 Plant monster on the field and draw 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// aux.FaceupFilter(Card.IsRace,RACE_PLANT)
fn faceup_plant(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsRace(RACE_PLANT) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && faceup_plant(d, chkc);
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //   and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_PLANT),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1)
            && duel::is_existing_target(d, Some(&faceup_plant), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_PLANT),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&faceup_plant), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsRace(RACE_PLANT) and Duel.Destroy(tc,REASON_EFFECT)>0
    //   and Duel.Draw(tp,1,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, e)
        && card::is_faceup(d, tc)
        && card::is_race(d, tc, RACE_PLANT)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::draw(d, tp, 1, REASON_EFFECT) > 0
    {
        // local dc=Duel.GetOperatedGroup():GetFirst()
        let og = duel::get_operated_group(d);
        let dc = group::get_first(d, &og).nil();
        // if dc:IsRace(RACE_PLANT) and not dc:IsPublic() and Duel.IsPlayerCanDraw(tp,1)
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if card::is_race(d, dc, RACE_PLANT)
            && !card::is_public(d, dc)
            && duel::is_player_can_draw(d, tp, 1)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ConfirmCards(1-tp,dc)
            duel::confirm_cards(d, 1 - tp, dc);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
