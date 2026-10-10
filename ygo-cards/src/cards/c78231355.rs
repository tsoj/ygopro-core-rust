//! Dragonmaid Hospitality
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 78231355;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGONMAID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRAGONMAID) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_DRAGONMAID as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.gyfilter(c,tc)`
fn gyfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsSetCard(SET_DRAGONMAID) and c:IsMonster() and c:IsAbleToGrave()
    //     and c:IsAttribute(tc:GetAttribute()) and not c:IsLevel(tc:GetLevel())
    card::is_set_card(d, c, &[SET_DRAGONMAID as u16])
        && card::is_monster(d, c)
        && card::is_able_to_grave(d, c, None)
        && {
            let attr = card::get_attribute(d, tc);
            card::is_attribute(d, c, attr)
        }
        && {
            let lv = card::get_level(d, tc);
            !card::is_level(d, c, &[lv])
        }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| filter(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0 {
            // local g=Duel.GetMatchingGroup(s.gyfilter,tp,LOCATION_DECK,0,nil,tc)
            let g = duel::get_matching_group(d, Some(&|d, c| gyfilter(d, c, tc)), a.tp, LOCATION_DECK, 0, NO_EX);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.SendtoGrave(sg,REASON_EFFECT)
                duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
            }
        }
    }
}
