//! Amaze Attraction Rapid Racing
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 55262310;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZEMENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // From cards_specific_functions.lua
    // aux.AddAttractionEquipProc(c)
    aux::add_attraction_equip_proc(d, c);
    // You: Target 1 card in your opponent's GY; change the equipped monster's battle position, and if you do, shuffle that target into the Deck.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_TODECK);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.AttractionEquipCon(true))
    effect::set_condition(d, e1, Some(aux::attraction_equip_con(true)));
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent: Increase the equipped monster's Level by 1 (until the end of this turn), and if you do, change its battle position.
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetProperty(0)
    effect::set_property(d, e2, 0, 0);
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_LVCHANGE);
    // e2:SetCondition(aux.AttractionEquipCon(false))
    effect::set_condition(d, e2, Some(aux::attraction_equip_con(false)));
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return not chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return !card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_able_to_deck(d, chkc);
    }
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,1,nil)
    //     and tc and tc:IsCanChangePosition() end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_able_to_deck(d, x)), tp, 0, LOCATION_GRAVE, 1, NO_EX)
            && tc.map_or(false, |tc| card::is_can_change_position(d, tc));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_able_to_deck(d, x)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // if ec and Duel.ChangePosition(ec,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)~=0
    //     and tc:IsRelateToEffect(e) then
    if let Some(ec) = ec {
        if duel::change_position(d, ec, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false) != 0
            && card::is_relate_to_effect(d, tc.nil(), a.e)
        {
            // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, tc.nil(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if chk==0 then return tc and tc:HasLevel() and tc:IsCanChangePosition() end
    if a.chk == Some(0) {
        return tc.map_or(false, |tc| card::has_level(d, tc) && card::is_can_change_position(d, tc));
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if ec and ec:IsFaceup() then
    if let Some(ec) = ec {
        if card::is_faceup(d, ec) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_LEVEL)
            effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // ec:RegisterEffect(e1)
            card::register_effect(d, ec, e1, false);
            // if not ec:IsImmuneToEffect(e1) then
            if !card::is_immune_to_effect(d, ec, e1) {
                // Duel.ChangePosition(ec,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)
                duel::change_position(d, ec, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
            }
        }
    }
}
