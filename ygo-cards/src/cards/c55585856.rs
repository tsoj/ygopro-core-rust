//! Misstic Call
use super::prelude::*;

pub const ID: u32 = 55585856;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand or GY: You can discard 1 other Spellcaster monster or 1 Spell/Trap; Special Summon this card, but banish it when it leaves the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.Discard(s.selfspcostfilter,true))
    effect::set_cost(
        d,
        e1,
        Some(cost::discard(
            Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| selfspcostfilter(d, c))),
            true,
            None,
            None,
            None,
        )),
    );
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Normal or Special Summoned: You can banish 1 Level 4 Spellcaster monster from your Deck, and if you do, during the End Phase of this turn, add it to your hand or Special Summon it
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2a, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetTarget(s.bantg)
    effect::set_target(d, e2a, Some(cost_target(bantg)));
    // e2a:SetOperation(s.banop)
    effect::set_operation(d, e2a, Some(operation(banop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.selfspcostfilter(c)`
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) or c:IsSpellTrap()
    card::is_race(d, c, RACE_SPELLCASTER) || card::is_spell_trap(d, c)
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // Banish it when it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
/// `s.banfilter(c)`
fn banfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(4) and c:IsRace(RACE_SPELLCASTER) and c:IsAbleToRemove()
    card::is_level(d, c, &[4]) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.banfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&banfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_REMOVED as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.banop(e,tp,eg,ep,ev,re,r,rp)`
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.banfilter,tp,LOCATION_DECK,0,1,1,nil)
    let rg = duel::select_matching_card(d, tp, Some(&banfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #rg==0 or Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)==0 or not rg:GetFirst():IsLocation(LOCATION_REMOVED) then return end
    if rg.is_empty() {
        return;
    }
    if duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let first = group::get_first(d, &rg).nil();
    if !card::is_location(d, first, LOCATION_REMOVED) {
        return;
    }
    // During the End Phase of this turn, add it to your hand or Special Summon it
    // aux.DelayedOperation(rg,PHASE_END,id,e,tp,function(ag)
    //     aux.ToHandOrElse(ag:GetFirst(),tp,
    //         function(tc)
    //             return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //         end,
    //         function(tc)
    //             Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
    //         end,
    //         aux.Stringid(id,2)
    //     )
    // end)
    let e = a.e;
    aux::delayed_operation(
        d,
        rg,
        PHASE_END,
        ID,
        e,
        tp,
        Some(std::sync::Arc::new(move |d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
            let tc = group::get_first(d, ag).nil();
            aux::to_hand_or_else(
                d,
                tc,
                tp,
                Some(&|d, tc| {
                    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                        && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
                }),
                Some(&|d, g: &CardSet| {
                    let tc = group::get_first(d, g).nil();
                    duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                    0
                }),
                Some(aux::stringid(ID, 2)),
            );
            true
        })),
        None,
        None,
        None,
        None,
        None,
    );
}
