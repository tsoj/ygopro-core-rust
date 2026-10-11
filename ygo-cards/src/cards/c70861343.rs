//! Ninjitsu Art of Transformation
use super::prelude::*;

pub const ID: u32 = 70861343;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // Destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,e,tp,ft)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return c:IsFaceup() and lv>0 and c:IsSetCard(SET_NINJA)
    //     and (ft>0 or (c:IsControler(tp) and c:GetSequence()<5))
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,lv+3,e,tp)
    card::is_faceup(d, c)
        && lv > 0
        && card::is_set_card(d, c, &[SET_NINJA as u16])
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, lv + 3, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.filter(c,lv,e,tp)`
fn filter(d: &mut Duel, c: CardId, lv: u32, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, lv)
        && card::is_race(d, c, (RACE_BEAST | RACE_WINGEDBEAST | RACE_INSECT) as u64)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        // return ft>-1 and Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,e,tp,ft)
        return ft > -1
            && duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.e, a.tp, ft)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,e,tp,ft)
    let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.e, a.tp, ft)), 1, 1, false, None, NO_EX);
    // e:SetLabel(rg:GetFirst():GetLevel()+3)
    let tc = group::get_first(d, &rg).nil();
    let lv = card::get_level(d, tc) as i64 + 3;
    effect::set_label(d, a.e, &[lv]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e:GetLabel(),e,tp)
    let lv = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, lv, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // c:SetCardTarget(tc)
            card::set_card_target(d, c, tc);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsLocation(LOCATION_MZONE) then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = card::get_first_card_target(d, h) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
