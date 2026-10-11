//! Mitsurugi Prayers
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 45171524;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MITSURUGI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply 1 of these effects, or if you Tributed a monster at activation, you can apply both effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DAMAGE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DAMAGE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.CheckReleaseGroupCost(tp,Card.IsRace,1,true,nil,nil,RACE_REPTILE) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if aux::check_release_group_cost(d, tp, Some(&|d, c| card::is_race(d, c, RACE_REPTILE)), 1, 1, true, None, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // local g=Duel.SelectReleaseGroupCost(tp,Card.IsRace,1,1,true,nil,nil,RACE_REPTILE)
        let g = aux::select_release_group_cost(d, tp, Some(&|d, c| card::is_race(d, c, RACE_REPTILE)), 1, 1, true, None, NO_EX);
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // e:SetLabel(1)
        effect::set_label(d, e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
    }
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MITSURUGI) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MITSURUGI as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MITSURUGI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MITSURUGI as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,1,tp,800)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, tp, 800);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local both=b1 and e:GetLabel()==1
    let both = b1 && effect::get_label(d, e) == 1;
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {true,aux.Stringid(id,3)},
    //     {both,aux.Stringid(id,4)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (true, aux::stringid(ID, 3)), (both, aux::stringid(ID, 4))]);
    // local breakeffect=false
    let mut breakeffect = false;
    // if op==1 or op==3 then
    if op == Some(1) || op == Some(3) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // breakeffect=true
            breakeffect = true;
        }
    }
    // if op==2 or op==3 then
    if op == Some(2) || op == Some(3) {
        // if breakeffect then Duel.BreakEffect() end
        if breakeffect {
            duel::break_effect(d);
        }
        // if Duel.Damage(tp,800,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
        let nvf = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
        if duel::damage(d, tp, 800, REASON_EFFECT, false, None) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&nvf), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 5))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
            let sel = duel::select_matching_card(d, tp, Some(&nvf), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(3207)
                effect::set_description(d, e1, 3207);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
                effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
