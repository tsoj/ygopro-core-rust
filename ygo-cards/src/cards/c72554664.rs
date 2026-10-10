//! Light of the Branded
use super::prelude::*;

pub const ID: u32 = 72554664;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return a Fusion monster to the Extra Deck
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn texfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:IsAbleToExtra()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION) && card::is_able_to_extra(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.texfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && texfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.texfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&texfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.texfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&texfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
fn albazfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_ALBAZ) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_ALBAZ]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, tc.nil(), e)
        && duel::send_to_deck(d, tc.nil(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc.nil(), LOCATION_EXTRA)
    {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return;
        }
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 or Duel.GetLocationCount(1-tp,LOCATION_MZONE)==0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0
            || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 == 0
        {
            return;
        }
        // local g1=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.albazfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
        let f1 = aux::necro_valley_filter(|d, c| albazfilter(d, c, e, tp));
        let g1 = duel::get_matching_group(d, Some(&f1), tp, LOCATION_GRAVE, 0, NO_EX);
        // local g2=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,0,LOCATION_GRAVE,nil,e,tp)
        let f2 = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
        let g2 = duel::get_matching_group(d, Some(&f2), tp, 0, LOCATION_GRAVE, NO_EX);
        // if #g1>0 and #g2>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g1.is_empty() && !g2.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sc1=g1:Select(tp,1,1,nil):GetFirst()
            let sel1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
            let sc1 = group::get_first(d, &sel1);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sc2=g2:Select(tp,1,1,nil):GetFirst()
            let sel2 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
            let sc2 = group::get_first(d, &sel2);
            // if sc1 and sc2 then
            if let (Some(sc1), Some(sc2)) = (sc1, sc2) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummonStep(sc1,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon_step(d, sc1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                // Duel.SpecialSummonStep(sc2,0,tp,1-tp,false,false,POS_FACEUP)
                duel::special_summon_step(d, sc2, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
