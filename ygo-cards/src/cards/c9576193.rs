//! Monster Slots
use super::prelude::*;

pub const ID: u32 = 9576193;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_REMOVE | CATEGORY_SPECIAL_SUMMON | CATEGORY_DRAW);
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
// function s.filter1(c,tp)
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsFaceup() and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,lv)
    lv > 0
        && card::is_faceup(d, c)
        && duel::is_existing_target(d, Some(&|d, x| filter2(d, x, lv)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.filter2(c,lv)
fn filter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsLevel(lv) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_level(d, c, &[lv]) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanSpecialSummon(tp)
    //     and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_draw(d, a.tp, 1)
            && duel::is_player_can_special_summon(d, a.tp, None)
            && duel::is_existing_target(d, Some(&|d, c| filter1(d, c, tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let tp = a.tp;
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,g1:GetFirst():GetLevel())
    let g1_first = group::get_first(d, &g1).nil();
    let lv = card::get_level(d, g1_first);
    let g2 = duel::select_target(d, a.tp, Some(&|d, x| filter2(d, x, lv)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    effect::set_label_object(d, a.e, Value::Card(g1_first));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g2), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local tc2=g:GetFirst()
    let mut tc2 = group::get_first(d, &g);
    // if tc2==tc1 then tc2=g:GetNext() end
    if tc2 == Some(tc1) {
        tc2 = group::iter(d, &g).get(1).copied();
    }
    // if tc1:IsFacedown() or not tc1:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc1) || !card::is_relate_to_effect(d, tc1, a.e) {
        return;
    }
    // if not tc2:IsRelateToEffect(e) or not tc2:IsLevel(tc1:GetLevel()) or Duel.Remove(tc2,POS_FACEUP,REASON_EFFECT)==0 then return end
    let tc2 = tc2.nil();
    if !card::is_relate_to_effect(d, tc2, a.e) {
        return;
    }
    let lv1 = card::get_level(d, tc1);
    if !card::is_level(d, tc2, &[lv1]) {
        return;
    }
    if duel::remove(d, tc2, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.Draw(tp,1,REASON_EFFECT)==0 then return end
    if duel::draw(d, a.tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    // local dr=Duel.GetOperatedGroup():GetFirst()
    let dr = group::get_first(d, &duel::get_operated_group(d)).nil();
    // Duel.ConfirmCards(1-tp,dr)
    duel::confirm_cards(d, 1 - a.tp, dr);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if dr:GetLevel()==tc1:GetLevel() then
    let dr_lv = card::get_level(d, dr);
    let tc1_lv = card::get_level(d, tc1);
    if dr_lv == tc1_lv {
        // if Duel.SpecialSummon(dr,0,tp,tp,false,false,POS_FACEUP)==0 then
        if duel::special_summon(d, dr, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
        }
    } else {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    }
}
