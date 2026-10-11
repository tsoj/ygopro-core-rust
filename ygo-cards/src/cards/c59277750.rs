//! Intruder Alarm - Yellow Alert
//! 敵襲警報－イエローアラート－
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 59277750;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| card::is_controler(d, duel::get_attacker(d).nil(), 1 - a.tp))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) Duel.SendtoHand(ag,nil,REASON_EFFECT) end
fn to_hand_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // --Return it to the hand at the end of the Battle Phase
            // aux.DelayedOperation(sc,PHASE_BATTLE,id,e,tp,function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,1))
            aux::delayed_operation(
                d,
                sc,
                PHASE_BATTLE,
                ID,
                a.e,
                a.tp,
                Some(Arc::new(to_hand_oper)),
                None,
                Some(0),
                Some(0),
                Some(aux::stringid(ID, 1)),
                None,
            );
            // --While you control that face-up Special Summoned monster, your opponent cannot target other monsters for attacks
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
            effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(0,LOCATION_MZONE)
            effect::set_target_range(d, e1, 0, LOCATION_MZONE);
            // e1:SetCondition(function() return sc:IsControler(tp) end)
            let tp = a.tp;
            effect::set_condition(d, e1, Some(condition(move |d, _a| card::is_controler(d, sc, tp))));
            // e1:SetValue(function(e,c) return c~=e:GetHandler() end)
            effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| c != effect::get_handler(d, e).nil()));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
