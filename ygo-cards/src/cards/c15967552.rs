//! Small Scuffle
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 15967552;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Level 2 or lower monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spfilter(c,e,tp,zone)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsLevelBelow(2) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE,tp,zone)
    card::is_level_below(d, c, 2) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), Some(tp), Some(zone))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local zones=0
    let mut zones: u32 = 0;
    // for seq=0,4 do
    for seq in 0..5u32 {
        // local z1=1<<seq
        let z1 = 1u32 << seq;
        // local z2=1<<(4-seq)
        let z2 = 1u32 << (4 - seq);
        // if Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,z1)>0
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(z1)).0 > 0
            // and Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp,LOCATION_REASON_TOFIELD,z2)>0
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(1 - a.tp), Some(LOCATION_REASON_TOFIELD), Some(z2)).0 > 0
            // and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,z1) then
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, z1)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        {
            // zones=zones|z1
            zones |= z1;
        }
    }
    // if chk==0 then return zones>0 end
    if a.chk == Some(0) {
        return zones > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local zone=Duel.SelectFieldZone(tp,1,LOCATION_MZONE,0,ZONES_EMZ|~zones)
    let zone = duel::select_field_zone(d, a.tp, 1, LOCATION_MZONE, 0, Some(ZONES_EMZ | !zones));
    // Duel.SetTargetParam(zone)
    duel::set_target_param(d, zone);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local z1=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let z1 = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // local z2=1<<(4-math.log(z1,2))
    // (z1 is a single zone, so its bit index is exact)
    let z2 = 1u32 << (4 - z1.trailing_zeros());
    // if Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,z1)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(z1)).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc1=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,z1):GetFirst()
    let g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, z1)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc1 = group::get_first(d, &g1);
    // if not sc1 or Duel.SpecialSummon(sc1,0,tp,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE,z1)==0 then return end
    let Some(sc1) = sc1 else { return };
    if duel::special_summon(d, sc1, 0, a.tp, a.tp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, Some(z1)) == 0 {
        return;
    }
    // if sc1:IsFacedown() then Duel.ConfirmCards(1-tp,sc1) end
    if card::is_facedown(d, sc1) {
        duel::confirm_cards(d, 1 - a.tp, sc1);
    }
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp,LOCATION_REASON_TOFIELD,z2)>0
    if duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(1 - a.tp), Some(LOCATION_REASON_TOFIELD), Some(z2)).0 > 0
        // and Duel.IsExistingMatchingCard(s.spfilter,1-tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,1-tp,z2)
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, 1 - a.tp, z2)), 1 - a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        // and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc2=Duel.SelectMatchingCard(1-tp,s.spfilter,1-tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,1-tp,z2):GetFirst()
        let g2 = duel::select_matching_card(d, 1 - a.tp, Some(&|d, c| spfilter(d, c, a.e, 1 - a.tp, z2)), 1 - a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if not sc2 then return end
        let Some(sc2) = group::get_first(d, &g2) else { return };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sc2,0,1-tp,1-tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE,z2)
        duel::special_summon(d, sc2, 0, 1 - a.tp, 1 - a.tp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, Some(z2));
        // if sc2:IsFacedown() then Duel.ConfirmCards(tp,sc2) end
        if card::is_facedown(d, sc2) {
            duel::confirm_cards(d, a.tp, sc2);
        }
    }
}
