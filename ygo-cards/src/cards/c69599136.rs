//! Floodgate Trap Hole
use super::prelude::*;

pub const ID: u32 = 69599136;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When opponent summons a monster(s), change it to face-down defense position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // e3:SetTarget(s.target2)
    effect::set_target(d, e3, Some(cost_target(target2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and not c:IsSummonPlayer(tp) and c:IsCanTurnSet()
    card::is_faceup(d, c) && !card::is_summon_player(d, c, tp) && card::is_can_turn_set(d, c)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg and eg:IsExists(s.filter,1,nil,tp) end
    if a.chk == Some(0) {
        return match &a.eg {
            Some(eg) => group::is_exists(d, eg, &|d, c| filter(d, c, a.tp), 1, NO_EX),
            None => false,
        };
    }
    // local g=eg:Filter(s.filter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
// s.target2(e,tp,eg,ep,ev,re,r,rp,chk)
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // if chk==0 then return rp==1-tp and tc:IsFaceup() and tc:IsCanTurnSet() end
    if a.chk == Some(0) {
        return a.rp == 1 - a.tp && card::is_faceup(d, tc.nil()) && card::is_can_turn_set(d, tc.nil());
    }
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,tc,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(tc.nil())), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)~=0 then
    if duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false) != 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local tc=og:GetFirst()
        let _ = group::get_first(d, &og);
        // for tc in aux.Next(og) do
        for tc in group::iter(d, &og) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3313)
            effect::set_description(d, e1, 3313);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
            effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
