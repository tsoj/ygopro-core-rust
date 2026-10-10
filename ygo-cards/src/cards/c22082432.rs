//! Dangers of the Divine
use super::prelude::*;

pub const ID: u32 = 22082432;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "The Winged Dragon of Ra" from your hand or banishment, ignoring its Summoning conditions
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(Cost.AND(Cost.PayLP(1/2),s.cost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::pay_lp(cost::PayLp::Fraction(0.5)), cost_target(cost)])));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Choose 1 "The Winged Dragon of Ra" in your Monster Zone and send all other monsters on the field to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_previous_location(d, h, LOCATION_ONFIELD)
        })),
    );
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// s.spcostfilter(c,tp)
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| spcostfilter(d, x, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,tp)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, x| spcostfilter(d, x, a.tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_RA) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[CARD_RA]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
/// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_REMOVED,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,true,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) {
            // local turn_count=Duel.GetTurnCount()
            let turn_count = duel::get_turn_count(d, None);
            // Make its ATK/DEF 4000
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_ATTACK);
            // e1:SetValue(4000)
            effect::set_value(d, e1, 4000);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
            // It cannot attack
            // local e3=e1:Clone()
            let e3 = effect::clone(d, e1);
            // e3:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
            // sc:RegisterEffect(e3)
            card::register_effect(d, sc, e3, false);
            // Return it to the hand during the End Phase of the next turn
            // aux.DelayedOperation(sc,PHASE_END,id,e,tp,
            //     function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end,
            //     function() return Duel.GetTurnCount()==turn_count+1 end,
            //     nil,2,aux.Stringid(id,2))
            aux::delayed_operation(
                d,
                sc,
                PHASE_END,
                ID,
                a.e,
                a.tp,
                Some(std::sync::Arc::new(|d, ag: &CardSet, _a: &EffArgs| {
                    duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
                    true
                })),
                Some(std::sync::Arc::new(move |d, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_count + 1)),
                None,
                Some(2),
                Some(aux::stringid(ID, 2)),
                None,
            );
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
/// s.selfilter(c)
fn selfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_RA) and c:IsFaceup() and Duel.IsExistingMatchingCard(Card.IsAbleToGrave,0,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_code(d, c, &[CARD_RA])
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELF as u64);
    // local g=Duel.SelectMatchingCard(tp,s.selfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&selfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // local og=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_MZONE,LOCATION_MZONE,g)
        let og = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, (&g).into());
        // if #og>0 then
        if !og.is_empty() {
            // Duel.SendtoGrave(og,REASON_EFFECT)
            duel::send_to_grave(d, og, REASON_EFFECT, None, None);
        }
    }
}
