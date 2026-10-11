//! Xyz Soul
use super::prelude::*;

pub const ID: u32 = 11228035;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 Xyz Monster in either player's Graveyard; all monsters you currently control gain ATK equal to its Rank x 200, then you can shuffle it into the Extra Deck. This ATK increase lasts until the End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_TOEXTRA)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_TOEXTRA);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(stat_change_condition)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.StatChangeDamageStepCondition`
fn stat_change_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsXyzMonster() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_xyz_monster(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsXyzMonster,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_xyz_monster), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsXyzMonster,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&card::is_xyz_monster), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOEXTRA,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA as u64, Some(g), 1, tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=tc:GetRank()*200
    let atk = card::get_rank(d, tc) as i64 * 200;
    // local atk_increase_chk=false
    let mut atk_increase_chk = false;
    // for ac in g:Iter() do
    for ac in group::iter(d, &g) {
        // local prev_atk=ac:GetAttack()
        let prev_atk = card::get_attack(d, ac);
        // ac:UpdateAttack(atk,RESETS_STANDARD_PHASE_END,c)
        card::update_attack(d, ac, atk, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
        // if not atk_increase_chk and ac:GetAttack()>prev_atk then
        if !atk_increase_chk && card::get_attack(d, ac) > prev_atk {
            // atk_increase_chk=true
            atk_increase_chk = true;
        }
    }
    // if atk_increase_chk and tc:IsAbleToDeck() and aux.nvfilter(tc)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if atk_increase_chk
        && card::is_able_to_deck(d, tc)
        && aux::nvfilter(d, tc)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
