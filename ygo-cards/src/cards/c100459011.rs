//! Penalty Game "GREED, the Illusion of Avarice"
//! 罰ゲーム「ＧＲＥＥＤ－欲望の幻像－」
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 100459011;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent added a card(s) from the Deck to the hand this turn, except during the Draw Phase: Inflict 400 damage to your opponent for each card in their hand, then if the total number of cards in their hand, field, and GY is 20 or more, you can banish their entire hand. You can only activate 1 "Penalty Game "GREED, the Illusion of Avarice"" per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.HasFlagEffect(1-tp,id) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::has_flag_effect(d, 1 - a.tp, ID, None))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_TOHAND|TIMING_CHAIN_END|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND | TIMING_CHAIN_END | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of cards being added from the Deck to a player's hand, except during the Draw Phase
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_HAND)
        effect::set_code(d, ge1, EVENT_TO_HAND);
        // ge1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return not Duel.IsDrawPhase() end)
        effect::set_condition(d, ge1, Some(condition(|d, _a| !duel::is_draw_phase(d, None))));
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
        effect::set_operation(d, ge1, Some(operation(ge1_operation)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}

/// The operation of ge1: `for ec in eg:Iter() do ... end`
fn ge1_operation(d: &mut Duel, a: &EffArgs) {
    // for ec in eg:Iter() do
    let eg = a.eg.clone().nil();
    for ec in group::iter(d, &eg) {
        // if ec:IsPreviousLocation(LOCATION_DECK) then
        if card::is_previous_location(d, ec, LOCATION_DECK) {
            // Duel.RegisterFlagEffect(ec:GetReasonPlayer(),id,RESET_PHASE|PHASE_END,0,1)
            let rp = card::get_reason_player(d, ec);
            duel::register_flag_effect(d, rp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}

/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local total_damage=400*Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let total_damage = 400 * duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) as i64;
    // if chk==0 then return total_damage>0 end
    if a.chk == Some(0) {
        return total_damage > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,total_damage)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, total_damage as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}

/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local total_damage=400*Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let total_damage = 400 * duel::get_field_group_count(d, tp, 0, LOCATION_HAND) as i64;
    // if total_damage>0 and Duel.Damage(1-tp,total_damage,REASON_EFFECT)>0
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)>=20
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if total_damage > 0
        && duel::damage(d, 1 - tp, total_damage, REASON_EFFECT, false, None) > 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) >= 20
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
        let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
