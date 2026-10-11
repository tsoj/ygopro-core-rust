//! Prima Light
use super::prelude::*;

pub const ID: u32 = 2507443;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
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
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 Warrior or Fairy "Cyber" monster from your Deck to your hand
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
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tribfilter(c,tp)`
fn tribfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and c:IsReleasableByEffect() and Duel.GetMZoneCount(tp,c)>0
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_releasable_by_effect(d, c, None, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_WARRIOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsSummonable(true,nil)
    card::is_race(d, c, RACE_WARRIOR) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.efftg(e,tp,...,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.CheckReleaseGroup(tp,s.tribfilter,1,nil,tp)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
    let tp = a.tp;
    let b1 = duel::check_release_group(d, tp, Some(&|d, c| tribfilter(d, c, tp)), 1, duel::ReleaseArgs::default(), NO_EX, false)
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsMainPhase() and Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND,0,1,nil)
    let b2 = duel::is_main_phase(d, None) && duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,2)},
    //   {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, tp, LOCATION_MZONE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    }
    true
}
/// `s.effop(e,tp,...)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // local rg=Duel.SelectReleaseGroup(tp,s.tribfilter,1,1,nil,tp)
        let rg = duel::select_release_group(d, tp, Some(&|d, c| tribfilter(d, c, tp)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false);
        let rg = rg.nil();
        // if #rg>0 and Duel.Release(rg,REASON_EFFECT)>0 then
        if !rg.is_empty() && duel::release(d, rg, REASON_EFFECT, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
            let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
        let sc = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sc);
        // if sc then
        if let Some(sc) = sc {
            // Duel.Summon(tp,sc,true,nil)
            duel::summon(d, tp, sc, true, None, 0, None);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR|RACE_FAIRY) and c:IsSetCard(SET_CYBER) and c:IsAbleToHand()
    card::is_race(d, c, RACE_WARRIOR | RACE_FAIRY) && card::is_set_card(d, c, &[SET_CYBER as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
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
