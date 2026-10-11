//! Mamemaki
//! 豆まき
use super::prelude::*;

pub const ID: u32 = 83828288;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DRAW + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c) as i64;
    // return lv>0 and c:IsFaceup() and c:IsAbleToDeck() and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>=lv
    //     and Duel.IsPlayerCanDraw(tp,lv)
    lv > 0
        && card::is_faceup(d, c)
        && card::is_able_to_deck(d, c)
        && duel::get_field_group_count(d, tp, LOCATION_HAND, 0) as i64 >= lv
        && duel::is_player_can_draw(d, tp, lv as u32)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local lv=g:GetFirst():GetLevel()
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first) as i64;
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,lv)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, lv as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,lv)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, lv as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc) as i64;
    // if tc:IsRelateToEffect(e) and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>=lv then
    if card::is_relate_to_effect(d, tc, a.e) && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) as i64 >= lv {
        // local ct=Duel.DiscardHand(tp,aux.TRUE,lv,lv,REASON_EFFECT|REASON_DISCARD)
        let ct = duel::discard_hand(d, a.tp, Some(&|_d: &mut Duel, _c: CardId| true), lv as u16, lv as u16, REASON_EFFECT | REASON_DISCARD, NO_EX);
        // if ct>0 and Duel.Draw(tp,lv,REASON_EFFECT)~=0 then
        if ct > 0 && duel::draw(d, a.tp, lv as u32, REASON_EFFECT) != 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
