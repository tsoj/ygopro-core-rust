//! Heavenly Dragon Circle
use super::prelude::*;

pub const ID: u32 = 51684157;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TENYI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to your hand, or Special Summon, 1 Wyrm monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Tenyi" card from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsNonEffectMonster),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_non_effect_monster(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX))),
    );
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.costfilter(c,e,tp)
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WYRM) and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:IsNonEffectMonster(),c)
    if !card::is_race(d, c, RACE_WYRM) {
        return false;
    }
    let non_eff = card::is_non_effect_monster(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thspfilter(d, x, e, tp, non_eff, Some(c))), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// s.thspfilter(c,e,tp,non_eff_chk,rc)
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, non_eff_chk: bool, rc: Option<CardId>) -> bool {
    // return c:IsRace(RACE_WYRM) and (c:IsAbleToHand()
    //     or (non_eff_chk and Duel.GetMZoneCount(tp,rc)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_race(d, c, RACE_WYRM)
        && (card::is_able_to_hand(d, c, None)
            || (non_eff_chk && {
                let ex = match rc {
                    Some(rc) => rc.into(),
                    None => NO_EX,
                };
                duel::get_m_zone_count(d, tp, ex, None, None, None).0 > 0
            } && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local rc=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &g).nil();
    // e:SetLabel(rc:IsNonEffectMonster() and 1 or 0)
    let label = if card::is_non_effect_monster(d, rc) { 1 } else { 0 };
    effect::set_label(d, a.e, &[label]);
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    true
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=label==100
        let cost_chk = label == 100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk or Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,false)
        return cost_chk || duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, false, None)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // if label==0 then
    if label == 0 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    } else {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sp_chk=e:GetLabel()==1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let sp_chk = effect::get_label(d, a.e) == 1 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local sel_string=sp_chk and aux.Stringid(id,2) or HINTMSG_ATOHAND
    let sel_string = if sp_chk { aux::stringid(ID, 2) } else { HINTMSG_ATOHAND as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,sel_string)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, sel_string);
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,sp_chk):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thspfilter(d, c, a.e, a.tp, sp_chk, None)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // if not sp_chk then
    if !sp_chk {
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - a.tp, sc);
    } else {
        // aux.ToHandOrElse(sc,tp,
        //     function() return sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //     function()
        //         if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        //             sc:NegateEffects(e:GetHandler())
        //         end
        //         Duel.SpecialSummonComplete()
        //     end,
        //     aux.Stringid(id,3)
        // )
        aux::to_hand_or_else(
            d,
            sc,
            a.tp,
            Some(&|d, _c| card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)),
            Some(&|d, _g: &CardSet| {
                if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
                    // sc:NegateEffects(e:GetHandler())
                    let h = effect::get_handler(d, a.e).nil();
                    card::negate_effects(d, sc, h, None, false, None);
                }
                duel::special_summon_complete(d);
                0
            }),
            Some(aux::stringid(ID, 3)),
        );
    }
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TENYI) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TENYI as u16]) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
