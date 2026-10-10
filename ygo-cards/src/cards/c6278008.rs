//! Starry Dragon's Cycle
use super::prelude::*;

pub const ID: u32 = 6278008;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 monster to the Extra Deck and Special Summon 1 appropriate monster from either GY
    // local e1=Effect.CreateEffect(c)
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
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
    // return c:IsFaceup() and c:IsType(TYPE_EXTRA) and c:IsAbleToExtra()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EXTRA) && card::is_able_to_extra(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.texfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && texfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.texfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&texfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectTarget(tp,s.texfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&texfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, card_type: u32, attr: u32, lvl: u32, rnk: u32, race: u64) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and ((card_type&TYPE_FUSION>0 and c:IsAttribute(attr)) or
    //     (card_type&TYPE_SYNCHRO>0 and c:HasLevel() and c:GetLevel()<lvl) or
    //     (card_type&TYPE_XYZ>0 and c:HasLevel() and c:IsLevel(rnk)) or
    //     (card_type&TYPE_LINK>0 and c:IsRace(race)))
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (((card_type & TYPE_FUSION) > 0 && card::is_attribute(d, c, attr))
            || ((card_type & TYPE_SYNCHRO) > 0 && card::has_level(d, c) && card::get_level(d, c) < lvl)
            || ((card_type & TYPE_XYZ) > 0 && card::has_level(d, c) && card::is_level(d, c, &[rnk]))
            || ((card_type & TYPE_LINK) > 0 && card::is_race(d, c, race)))
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_EXTRA)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,
        //     tc:GetType(),tc:GetAttribute(),tc:GetLevel(),tc:GetRank(),tc:GetRace())
        let ty = card::get_type(d, tc);
        let attr = card::get_attribute(d, tc);
        let lvl = card::get_level(d, tc);
        let rnk = card::get_rank(d, tc);
        let race = card::get_race(d, tc);
        let (e, tp) = (a.e, a.tp);
        let filter = aux::necro_valley_filter(move |d: &mut Duel, x| spfilter(d, x, e, tp, ty, attr, lvl, rnk, race));
        let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
