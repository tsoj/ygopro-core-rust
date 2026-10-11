//! Fuh-Rin-Ka-Zan
use super::prelude::*;

pub const ID: u32 = 1781310;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_HANDES + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_TOHAND)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_TOHAND));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WIND),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_FIRE),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_EARTH),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let tp = a.tp;
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WIND)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let tp = a.tp;
        // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_MZONE,1,nil)
        //     or Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil)
        //     or Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>=2
        //     or Duel.IsPlayerCanDraw(tp,2) end
        return duel::is_existing_matching_card(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            || duel::get_field_group_count(d, tp, 0, LOCATION_HAND) >= 2
            || duel::is_player_can_draw(d, tp, 2);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local off=1
    // local ops={}
    // local opval={}
    let mut ops: Vec<u64> = Vec::new();
    let mut opval: Vec<i32> = Vec::new();
    // if Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) then
    if duel::is_existing_matching_card(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // ops[off]=aux.Stringid(id,0)
        // opval[off-1]=1
        ops.push(aux::stringid(ID, 0));
        opval.push(1);
    }
    // if Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil) then
    if duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, NO_EX) {
        ops.push(aux::stringid(ID, 1));
        opval.push(2);
    }
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>=2 then
    if duel::get_field_group_count(d, tp, 0, LOCATION_HAND) >= 2 {
        ops.push(aux::stringid(ID, 2));
        opval.push(3);
    }
    // if Duel.IsPlayerCanDraw(tp,2) then
    if duel::is_player_can_draw(d, tp, 2) {
        ops.push(aux::stringid(ID, 3));
        opval.push(4);
    }
    // if off==1 then return end
    if ops.is_empty() {
        return;
    }
    // local op=Duel.SelectOption(tp,table.unpack(ops))
    let sel = duel::select_option(d, tp, true, &ops);
    // if opval[op]==1 then
    let val = opval[sel as usize];
    if val == 1 {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else if val == 2 {
        // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else if val == 3 {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(1-tp,2)
        let hand = duel::get_field_group(d, tp, 0, LOCATION_HAND);
        let g = group::random_select(d, &hand, 1 - tp, 2);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    } else if val == 4 {
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
