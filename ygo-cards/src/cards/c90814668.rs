//! Tyrant Farm
use super::prelude::*;

pub const ID: u32 = 90814668;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 non-effect monster from GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_EFFECT) and Duel.GetMZoneCount(tp,c)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c)
    card::is_type(d, c, TYPE_EFFECT)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,tc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: Option<CardId>) -> bool {
    // return c:IsNonEffectMonster() and c:GetOriginalRace()==tc:GetOriginalRace()
    //     and c:GetOriginalAttribute()==tc:GetOriginalAttribute() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    // (tc is nil in the Lua when the filter is called without a card: indexing it is an error)
    let tc = tc.nil();
    card::is_non_effect_monster(d, c)
        && card::get_original_race(d, c) == card::get_original_race(d, tc)
        && card::get_original_attribute(d, c) == card::get_original_attribute(d, tc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
/// The Lua has a `chkc` line here, but this function has no `chkc` parameter, so `chkc` is the
/// global (nil) and that line is never taken.
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
        //     and Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp)
        let (e, tp) = (a.e, a.tp);
        return effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local bg=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let bg = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(bg,REASON_COST)
    duel::release(d, bg.clone(), REASON_COST, None);
    // Duel.SetTargetCard(bg)
    duel::set_target_card(d, bg);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp,tc)
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, tc));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
