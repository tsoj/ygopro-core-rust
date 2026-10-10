//! Stellar Wind Wolfrayet
use super::prelude::*;

pub const ID: u32 = 3322931;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Only control 1 of this card
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Must properly Summon before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Gain ATK when monster effect is activated
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetCode(EVENT_CHAINING)
    effect::set_code(d, e0, EVENT_CHAINING);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle all opponent monsters and this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // if not re:IsMonsterEffect() or re:GetHandler()==c then return end
    if !effect::is_monster_effect(d, re) || effect::get_handler(d, re) == Some(c) {
        return;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, 0, 0);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // if not re:IsMonsterEffect() or re:GetHandler()==c then return false end
    if !effect::is_monster_effect(d, re) || effect::get_handler(d, re) == Some(c) {
        return false;
    }
    // return c:GetFlagEffect(id)>0 and c:GetAttack()<4000
    card::get_flag_effect(d, c, ID) > 0 && card::get_attack(d, c) < 4000
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
    effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tdcon(e)
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsAttackAbove(4000) and c:IsAbleToDeck()
    card::is_attack_above(d, c, 4000) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // g:AddCard(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), g.len() as u8, 0, LOCATION_MZONE as i32);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if not c:IsRelateToEffect(e) or #g==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || g.is_empty() {
        return;
    }
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, &g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
