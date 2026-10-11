//! Mischief of the Wolves
use super::prelude::*;

pub const ID: u32 = 95589901;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply these effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase, if this card is in your GY because it was sent there this turn: You can target 1 face-up card you control; destroy it, and if you do, add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():GetTurnID()==Duel.GetTurnCount() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn opp_levelabove3(d: &mut Duel, c: CardId) -> bool {
    // aux.FaceupFilter(Card.IsLevelAbove,3)
    card::is_faceup(d, c) && card::is_level_above(d, c, 3)
}
fn has_level_faceup(d: &mut Duel, c: CardId) -> bool {
    // aux.FaceupFilter(Card.HasLevel)
    card::is_faceup(d, c) && card::has_level(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevelAbove,3),tp,0,LOCATION_MZONE,1,nil)
    //   or Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.HasLevel),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&opp_levelabove3), tp, 0, LOCATION_MZONE, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&has_level_faceup), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local break_chk=false
    let mut break_chk = false;
    // local opp_g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelAbove,3),tp,0,LOCATION_MZONE,nil)
    let opp_g = duel::get_matching_group(d, Some(&opp_levelabove3), tp, 0, LOCATION_MZONE, NO_EX);
    // if #opp_g>0 then
    if !opp_g.is_empty() {
        // break_chk=true
        break_chk = true;
        // for opp_c in opp_g:Iter() do
        for opp_c in group::iter(d, &opp_g) {
            // --● Reduce the Levels of all monsters your opponent controls by 2
            // opp_c:UpdateLevel(-2,nil,c)
            card::update_level(d, opp_c, -2, None, Some(c));
        }
    }
    // local your_g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.HasLevel),tp,LOCATION_MZONE,0,nil)
    let your_g = duel::get_matching_group(d, Some(&has_level_faceup), tp, LOCATION_MZONE, 0, NO_EX);
    // if #your_g>0 then
    if !your_g.is_empty() {
        // if break_chk then Duel.BreakEffect() end
        if break_chk {
            duel::break_effect(d);
        }
        // for your_c in your_g:Iter() do
        for your_c in group::iter(d, &your_g) {
            // --● Increase the Levels of all monsters you control by 2
            // your_c:UpdateLevel(2,nil,c)
            card::update_level(d, your_c, 2, None, Some(c));
        }
    }
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetTurnID()==Duel.GetTurnCount()
    let h = effect::get_handler(d, a.e).nil();
    card::get_turn_id(d, h) as i32 == duel::get_turn_count(d, None)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && duel::is_existing_target(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
