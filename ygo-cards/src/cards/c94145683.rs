//! Swallow's Nest
use super::prelude::*;

pub const ID: u32 = 94145683;

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
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.filter1(c,e,tp,ft)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsFaceup() and c:IsRace(RACE_WINGEDBEAST)
    //     and (ft>0 or (c:IsControler(tp) and c:GetSequence()<5)) and (c:IsControler(tp) or c:IsFaceup())
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil,lv,e,tp)
    lv > 0
        && card::is_faceup(d, c)
        && card::is_race(d, c, RACE_WINGEDBEAST)
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, lv, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.filter2(c,lv,e,tp)`
fn filter2(d: &mut Duel, c: CardId, lv: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:GetLevel()==lv and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WINGEDBEAST) && card::get_level(d, c) == lv && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return ft>-1 and Duel.CheckReleaseGroup(tp,s.filter1,1,nil,e,tp,ft)
        return ft > -1 && duel::check_release_group(d, tp, Some(&|d, c| filter1(d, c, e, tp, ft)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local rg=Duel.SelectReleaseGroup(tp,s.filter1,1,1,nil,e,tp,ft)
    let rg = duel::select_release_group(d, tp, Some(&|d, c| filter1(d, c, e, tp, ft)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // e:SetLabel(rg:GetFirst():GetLevel())
    let first = group::get_first(d, &rg).nil();
    let lv = card::get_level(d, first);
    effect::set_label(d, e, &[lv as i64]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, e) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,lv,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, lv, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
