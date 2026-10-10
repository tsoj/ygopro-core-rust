//! Gemini Ablation
use super::prelude::*;

pub const ID: u32 = 80758812;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_GEMINI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Gemini monster from the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 FIRE Warrior monster from hand or Deck
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_RELEASE)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_RELEASE);
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg2)
    effect::set_target(d, e3, Some(cost_target(sptg2)));
    // e3:SetOperation(s.spop2)
    effect::set_operation(d, e3, Some(operation(spop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// Discard 1 card as cost
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_DISCARD+REASON_COST,nil)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_DISCARD | REASON_COST, NO_EX);
    true
}
/// Check for a Gemini monster
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_GEMINI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    card::is_type(d, c, TYPE_GEMINI) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
/// Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// Special Summon 1 Gemini monster from the Deck, it gains its effects
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // tc:EnableGeminiStatus()
        card::enable_gemini_status(d, tc);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// "Cost"
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsDiscardable() and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_HAND|LOCATION_DECK,0,1,c,e,tp)
    card::is_discardable(d, c, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, c.into())
}
/// Check for a FIRE Warrior monster
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsMonster() and c:IsAttribute(ATTRIBUTE_FIRE)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// Activation legality
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=0 then
        if effect::get_label(d, e) != 0 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // If you only have 1 FIRE Warrior monster and it's in your hand, don't discard it
            // return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
            //   and Duel.IsExistingMatchingCard(s.releasefil,tp,LOCATION_MZONE,0,1,nil)
            return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
                && duel::is_existing_matching_card(d, Some(&releasefil), tp, LOCATION_MZONE, 0, 1, NO_EX);
        } else {
            // return Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
            //   and Duel.IsExistingMatchingCard(s.releasefil,tp,LOCATION_MZONE,0,1,nil)
            return duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
                && duel::is_existing_matching_card(d, Some(&releasefil), tp, LOCATION_MZONE, 0, 1, NO_EX);
        }
    }
    // if e:GetLabel()~=0 then
    if effect::get_label(d, e) != 0 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
        duel::send_to_grave(d, &g, REASON_COST | REASON_DISCARD, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// Check for a Gemini monster to tribute
fn releasefil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_GEMINI) and c:IsReleasableByEffect()
    card::is_type(d, c, TYPE_GEMINI) && card::is_releasable_by_effect(d, c, None, None)
}
/// Tribute 1 Gemini monster, Special Summon a FIRE Warrior, destroy a card if the tributed was an effect monster
fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.releasefil,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&releasefil), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    // local gemini_chk=tc:IsFaceup() and tc:IsGeminiStatus()
    let gemini_chk = card::is_faceup(d, tc) && card::is_gemini_status(d, tc);
    // if Duel.Release(tc,REASON_EFFECT)==0 then return end
    if duel::release(d, tc, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #sg==0 or Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)==0 or not gemini_chk then return end
    if sg.is_empty() || duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 || !gemini_chk {
        return;
    }
    // local dg1=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let dg1 = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #dg1==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if group::get_count(&dg1) == 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg2=dg1:Select(tp,1,1,nil)
    let dg2 = group::select(d, &dg1, tp, 1, 1, false, NO_EX).nil();
    // if #dg2>0 then
    if !dg2.is_empty() {
        // Duel.HintSelection(dg2,true)
        duel::hint_selection(d, &dg2, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(dg2,REASON_EFFECT)
        duel::destroy(d, &dg2, REASON_EFFECT, None, None);
    }
}
