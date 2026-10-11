//! Arbitration of White
use super::prelude::*;

pub const ID: u32 = 57964143;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Treat a Fish monster as a Tuner
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(tunertg)));
    effect::set_operation(d, e2, Some(operation(tunerop)));
    card::register_effect(d, c, e2, false);
    // Change all monsters the opponent controls to Defense position
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_POSITION);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(cond3)));
    effect::set_cost(d, e3, Some(cost_target(poscost)));
    effect::set_target(d, e3, Some(cost_target(postg)));
    effect::set_operation(d, e3, Some(operation(posop)));
    card::register_effect(d, c, e3, false);
    // Special Summon 1 Fish monster from the GY
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    effect::set_condition(d, e4, Some(condition(spcon1)));
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    effect::set_condition(d, e5, Some(condition(spcon2)));
    card::register_effect(d, c, e5, false);
}
fn tunerfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FISH) and not c:IsType(TYPE_TUNER)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FISH) && !card::is_type(d, c, TYPE_TUNER)
}
fn tunertg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tunerfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tunerfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tunerfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tunerfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.tunerfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&tunerfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn tunerop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        effect::set_value(d, e1, TYPE_TUNER as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
fn cstfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsDiscardable()
    card::is_race(d, c, RACE_FISH) && card::is_discardable(d, c, None, None)
}
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
}
fn poscost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cstfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cstfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cstfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&cstfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsCanChangePosition()
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_can_change_position(d, c)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.posfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&posfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.posfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&posfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.posfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&posfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
    duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
}
fn spconfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:GetPreviousRaceOnField()&RACE_FISH==RACE_FISH
    card::is_previous_controler(d, c, tp) && (card::get_previous_race_on_field(d, c) & RACE_FISH) == RACE_FISH
}
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter1,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spconfilter1(d, c, a.tp), 1, NO_EX)
}
fn spconfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsPreviousControler(tp) and not c:IsReason(REASON_BATTLE)
    card::is_race(d, c, RACE_FISH) && card::is_previous_controler(d, c, tp) && !card::is_reason(d, c, REASON_BATTLE)
}
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter2,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spconfilter2(d, c, a.tp), 1, NO_EX)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FISH) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
