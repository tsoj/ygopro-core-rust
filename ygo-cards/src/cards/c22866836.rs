//! Spin Turn
use super::prelude::*;

pub const ID: u32 = 22866836;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 Attack Position EARTH Machine Xyz monster you control and 1 monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.operation1)
    effect::set_operation(d, e1, Some(operation(operation1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Target 1 Defense Position EARTH Machine Xyz monster you control and 1 Spell/Trap your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation2)
    effect::set_operation(d, e2, Some(operation(operation2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.mfilter1(c)
fn mfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsCanChangePosition() and c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsType(TYPE_XYZ)
    card::is_position(d, c, POS_FACEUP_ATTACK as u32)
        && card::is_can_change_position(d, c)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_type(d, c, TYPE_XYZ)
}
// s.mfilter2(c)
fn mfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_DEFENSE) and c:IsCanChangePosition() and c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsType(TYPE_XYZ)
    card::is_position(d, c, POS_FACEUP_DEFENSE as u32)
        && card::is_can_change_position(d, c)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_type(d, c, TYPE_XYZ)
}
// s.target1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    target_common(d, a, &mfilter1, LOCATION_MZONE)
}
// s.target2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    target_common(d, a, &mfilter2, LOCATION_SZONE)
}
// Shared body of s.target1 (opponent's monsters, LOCATION_MZONE) and s.target2 (opponent's Spell/Traps, LOCATION_SZONE)
fn target_common(d: &mut Duel, a: &EffArgs, mfilter: &dyn Fn(&mut Duel, CardId) -> bool, opp_zone: u32) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.mfilter1,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(mfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|_, _| true), a.tp, 0, opp_zone, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACK as u64);
    // local g1=Duel.SelectTarget(tp,s.mfilter1,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(mfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1).map_or(Value::Nil, Value::Card);
    effect::set_label_object(d, a.e, first);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g2=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&|_, _| true), a.tp, 0, opp_zone, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g1), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g2), 1, 0, 0);
    true
}
// s.operation1(e)
fn operation1(d: &mut Duel, a: &EffArgs) {
    resolve(d, a);
}
// s.operation2(e)
fn operation2(d: &mut Duel, a: &EffArgs) {
    resolve(d, a);
}
// Shared body of s.operation1 and s.operation2
fn resolve(d: &mut Duel, a: &EffArgs) {
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local tc2=g:GetFirst()
    let order = group::iter(d, &g);
    let mut tc2 = order.first().copied();
    // if tc2==tc1 then tc2=g:GetNext() end
    if tc2 == tc1 {
        tc2 = order.get(1).copied();
    }
    // if tc1:IsRelateToEffect(e) and tc1:IsFaceup() and Duel.ChangePosition(tc1,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)~=0 and tc2:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc1.nil(), a.e)
        && card::is_faceup(d, tc1.nil())
        && duel::change_position(d, CardSet::from_card(tc1.nil()), POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false) != 0
        && card::is_relate_to_effect(d, tc2.nil(), a.e)
    {
        // Duel.Destroy(tc2,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc2.nil()), REASON_EFFECT, None, None);
    }
}
