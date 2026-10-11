//! Five Star Twilight
use super::prelude::*;

pub const ID: u32 = 6309986;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[44632120, 71036835, 7021574, 34419588, CARD_KURIBOH], ..ScriptInfo::DEFAULT };

/// `local locs=LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE`
const LOCS: u32 = LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // return #g==1 and g:GetFirst():IsLevel(5) and g:GetFirst():IsFaceup()
    g.len() == 1 && {
        let tc = group::get_first(d, &g).nil();
        card::is_level(d, tc, &[5]) && card::is_faceup(d, tc)
    }
}
/// `s.cfilter(c,ft,tp)`
fn cfilter(d: &mut Duel, c: CardId, ft: i64, tp: u8) -> bool {
    // return c:IsLevel(5) and (ft>=5 or (c:IsControler(tp) and c:GetSequence()<5)) and (c:IsControler(tp) or c:IsFaceup())
    card::is_level(d, c, &[5])
        && (ft >= 5 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // if chk==0 then return ft>=4 and Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,ft,tp) end
    if a.chk == Some(0) {
        return ft >= 4 && duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,ft,tp)
    let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.filter(c,e,tp,code)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)<5 then return false end
        if effect::get_label(d, a.e) == 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 5 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //   and Duel.IsExistingMatchingCard(s.filter,tp,locs,0,1,nil,e,tp,44632120)
        //   and ... (71036835, 7021574, 34419588, CARD_KURIBOH)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && has_code(d, a, 44632120)
            && has_code(d, a, 71036835)
            && has_code(d, a, 7021574)
            && has_code(d, a, 34419588)
            && has_code(d, a, CARD_KURIBOH);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,5,tp,locs)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 5, a.tp, LOCS as i32);
    true
}
/// `Duel.IsExistingMatchingCard(s.filter,tp,locs,0,1,nil,e,tp,code)`
fn has_code(d: &mut Duel, a: &EffArgs, code: u32) -> bool {
    let (e, tp) = (a.e, a.tp);
    duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, code)), tp, LOCS, 0, 1, NO_EX)
}
/// `Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,locs,0,nil,e,tp,code)`
fn necro_group(d: &mut Duel, a: &EffArgs, code: u32) -> CardSet {
    let (e, tp) = (a.e, a.tp);
    duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| filter(d, c, e, tp, code))), tp, LOCS, 0, NO_EX)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<5 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 5
        || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // local g1..g5=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),...)
    let g1 = necro_group(d, a, 44632120);
    let g2 = necro_group(d, a, 71036835);
    let g3 = necro_group(d, a, 7021574);
    let g4 = necro_group(d, a, 34419588);
    let g5 = necro_group(d, a, CARD_KURIBOH);
    // if #g1>0 and #g2>0 and #g3>0 and #g4>0 and #g5>0 then
    if !g1.is_empty() && !g2.is_empty() && !g3.is_empty() && !g4.is_empty() && !g5.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        // local sg1=g1:Select(tp,1,1,nil)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let mut sg1 = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        // local sg2=g2:Select(tp,1,1,nil)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg2 = group::select(d, &g2, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        // local sg3=g3:Select(tp,1,1,nil)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg3 = group::select(d, &g3, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg3)
        group::add_card(&mut sg1, &sg3);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        // local sg4=g4:Select(tp,1,1,nil)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg4 = group::select(d, &g4, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg4)
        group::add_card(&mut sg1, &sg4);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        // local sg5=g5:Select(tp,1,1,nil)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg5 = group::select(d, &g5, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg5)
        group::add_card(&mut sg1, &sg5);
        // for tc in aux.Next(sg1) do
        for tc in group::iter(d, &sg1) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
            effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
