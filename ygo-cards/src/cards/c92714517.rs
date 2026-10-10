//! Big Welcome Labrynth
use super::prelude::*;

pub const ID: u32 = 92714517;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LABRYNTH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Labrynth" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 Fiend monster to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_LABRYNTH) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_LABRYNTH as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSendtoHand(tp)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_send_to_hand(d, a.tp, None)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let (e, tp) = (a.e, a.tp);
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))),
            a.tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
        if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
            // local hg=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_MZONE,0,1,1,nil)
            let hg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            // if #hg>0 then
            if !hg.is_empty() {
                // Duel.HintSelection(hg,true)
                duel::hint_selection(d, hg.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(hg,nil,REASON_EFFECT)
                duel::send_to_hand(d, hg, None, REASON_EFFECT, None);
            }
        }
    }
    // aux.WelcomeLabrynthTrapDestroyOperation(e,tp)
    aux::welcome_labrynth_trap_destroy_operation(d, a.e, a.tp);
}
// function s.thfilter(c,tp,fnd_chk)
fn thfilter(d: &mut Duel, c: CardId, tp: u8, fnd_chk: bool) -> bool {
    // if not c:IsAbleToHand() then return false end
    if !card::is_able_to_hand(d, c, None) {
        return false;
    }
    // if c:IsControler(1-tp) then return fnd_chk end
    if card::is_controler(d, c, 1 - tp) {
        return fnd_chk;
    }
    // return c:IsFaceup() and c:IsRace(RACE_FIEND)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND)
}
// function s.fndfilter(c)
fn fndfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(8) and c:IsRace(RACE_FIEND)
    card::is_faceup(d, c) && card::is_level_above(d, c, 8) && card::is_race(d, c, RACE_FIEND)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local fnd_chk=Duel.IsExistingMatchingCard(s.fndfilter,tp,LOCATION_MZONE,0,1,nil)
    let fnd_chk = duel::is_existing_matching_card(d, Some(&fndfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chkc then return s.thfilter(chkc,tp,fnd_chk) end
    // (the Lua `chkc` is not a parameter of this function: it is the global nil, so this branch never runs)
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_MZONE,LOCATION_ONFIELD,1,nil,tp,fnd_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| thfilter(d, c, a.tp, fnd_chk)), a.tp, LOCATION_MZONE, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_MZONE,LOCATION_ONFIELD,1,1,nil,tp,fnd_chk)
    let g = duel::select_target(d, a.tp, Some(&|d, c| thfilter(d, c, a.tp, fnd_chk)), a.tp, LOCATION_MZONE, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
