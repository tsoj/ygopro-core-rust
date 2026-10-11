//! Comic Cat
use super::prelude::*;

pub const ID: u32 = 72921536;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card is treated as a Toon monster while "Toon World" is on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e1, EFFECT_ADD_TYPE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_TOON_WORLD),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end)
    effect::set_condition(d, e1, Some(condition(|d, _a| {
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), 0, LOCATION_ONFIELD as u32, LOCATION_ONFIELD as u32, 1, NO_EX)
    })));
    // e1:SetValue(TYPE_TOON)
    effect::set_value(d, e1, TYPE_TOON as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // During the Main Phase (Quick Effect): You can Tribute 1 monster (if you control "Toon World", you can Tribute 1 monster your opponent controls, even though you do not control it), and if you do, Special Summon 1 monster that mentions "Toon World" from your hand or Deck, ignoring its Summoning conditions
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_RELEASE | CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn tribfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReleasableByEffect() and Duel.GetMZoneCount(tp,c)>0
    card::is_releasable_by_effect(d, c, None, None) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:ListsCode(CARD_TOON_WORLD) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::lists_code(d, c, &[CARD_TOON_WORLD]) && card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn opp_location(d: &mut Duel, tp: u8) -> u32 {
    // local opp_location=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_TOON_WORLD),tp,LOCATION_ONFIELD,0,1,nil) and LOCATION_MZONE or 0
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), tp, LOCATION_ONFIELD as u32, 0, 1, NO_EX) {
        LOCATION_MZONE as u32
    } else {
        0
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let loc = opp_location(d, a.tp);
        return duel::is_existing_matching_card(d, Some(&|d, c| tribfilter(d, c, a.tp)), a.tp, LOCATION_MZONE as u32, loc, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, (LOCATION_HAND | LOCATION_DECK) as u32, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local opp_location=...
    let loc = opp_location(d, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.tribfilter,tp,LOCATION_MZONE,opp_location,1,1,nil,tp)
    let rg = duel::select_matching_card(d, a.tp, Some(&|d, c| tribfilter(d, c, a.tp)), a.tp, LOCATION_MZONE as u32, loc, 1, 1, false, NO_EX).nil();
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // Duel.HintSelection(rg)
    duel::hint_selection(d, rg.clone(), true);
    // if Duel.Release(rg,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::release(d, rg, REASON_EFFECT, None) > 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,HAND|DECK,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, (LOCATION_HAND | LOCATION_DECK) as u32, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
