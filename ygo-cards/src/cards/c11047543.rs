//! Psychic Feel Zone
use super::prelude::*;

pub const ID: u32 = 11047543;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter1(c,e,tp)
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_PSYCHIC) and c:IsType(TYPE_TUNER)
    //   and Duel.IsExistingTarget(s.filter2,tp,LOCATION_REMOVED,0,1,nil,e,tp,c:GetLevel())
    if !(card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC) && card::is_type(d, c, TYPE_TUNER)) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_existing_target(d, Some(&|d, x| filter2(d, x, e, tp, lv)), tp, LOCATION_REMOVED, 0, 1, NO_EX)
}
// function s.filter2(c,e,tp,lv)
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // local clv=c:GetLevel()
    let clv = card::get_level(d, c);
    // return clv>0 and c:IsFaceup() and c:IsRace(RACE_PSYCHIC) and not c:IsType(TYPE_TUNER)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,lv+clv)
    if !(clv > 0 && card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC) && !card::is_type(d, c, TYPE_TUNER)) {
        return false;
    }
    let sum = lv + clv;
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, sum)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,lv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsRace(RACE_PSYCHIC) and c:IsType(TYPE_SYNCHRO) and c:GetLevel()==lv and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_race(d, c, RACE_PSYCHIC)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::get_level(d, c) == lv
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let mut g1 = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_REMOVED,0,1,1,nil,e,tp,g1:GetFirst():GetLevel())
    let tc1 = group::get_first(d, &g1).nil();
    let lv = card::get_level(d, tc1);
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, a.e, a.tp, lv)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g1), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc1=g:GetFirst()
    // local tc2=g:GetNext()
    let order = group::iter(d, &g);
    let tc1 = order.first().copied().nil();
    let tc2 = order.get(1).copied();
    // if not tc1:IsRelateToEffect(e) or not tc2:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc1, a.e) || !card::is_relate_to_effect(d, tc2.nil(), a.e) {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp,tc1:GetLevel()+tc2:GetLevel())
    let lv = card::get_level(d, tc1) + card::get_level(d, tc2.nil());
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, lv)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.SendtoGrave(g,REASON_EFFECT|REASON_RETURN)
    duel::send_to_grave(d, g.clone(), REASON_EFFECT | REASON_RETURN, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local ssg=sg:Select(tp,1,1,nil)
    let ssg = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
    // Duel.SpecialSummon(ssg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
    duel::special_summon(d, ssg, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
