//! Summon Dice
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 41139112;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_SUMMON+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_SUMMON + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonable(true,nil)
    card::is_summonable(d, c, true, None, 0, None)
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.filter3(c,e,tp)`
fn filter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(5) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 5) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local d=Duel.TossDice(tp,1)
    let dice = duel::toss_dice(d, a.tp, 1, 0)[0];
    // if d==1 or d==2 then
    if dice == 1 || dice == 2 {
        // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| filter1(d, c)), a.tp, LOCATION_HAND, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // Duel.Summon(tp,tc,true,nil)
            duel::summon(d, a.tp, tc, true, None, 0, None);
        }
    }
    // elseif d==3 or d==4 then
    else if dice == 3 || dice == 4 {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter2),tp,LOCATION_GRAVE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| filter2(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // else
    else {
        // local g=Duel.GetMatchingGroup(s.filter3,tp,LOCATION_HAND,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| filter3(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
