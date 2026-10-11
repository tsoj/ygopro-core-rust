//! Dracoon Lamp
use super::prelude::*;

pub const ID: u32 = 42596828;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.defcost)
    effect::set_cost(d, e1, Some(cost_target(defcost)));
    // e1:SetOperation(s.defop)
    effect::set_operation(d, e1, Some(operation(defop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change stats
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.defcfilter(c)`
fn defcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WYRM) and c:IsDiscardable()
    card::is_race(d, c, RACE_WYRM as u64) && card::is_discardable(d, c, None, None)
}
/// `s.defcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn defcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.defcfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&defcfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.defcfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&defcfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.defop(e,tp,eg,ep,ev,re,r,rp)`
fn defop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return bc~=nil and c:IsAttackPos() and bc:IsAttackPos() and not bc:IsType(TYPE_LINK)
    match bc {
        Some(bc) => card::is_attack_pos(d, c) && card::is_attack_pos(d, bc) && !card::is_type(d, bc, TYPE_LINK),
        None => false,
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if c:IsRelateToBattle() and bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, c) && card::is_relate_to_battle(d, bc) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_CHANGE_BATTLE_STAT)
            effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_STAT);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
            effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
            // e1:SetTarget(function(e2,c2) return c2==c or c2==bc end)
            effect::set_target(d, e1, Some(effect_card_fn(move |_d, _e, c2| c2 == c || c2 == bc)));
            // e1:SetValue(function(e3,c3) return c3:GetDefense() end)
            effect::set_value_fn(d, e1, effect_card_value(|d, _e, c3| card::get_defense(d, c3) as i64));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
