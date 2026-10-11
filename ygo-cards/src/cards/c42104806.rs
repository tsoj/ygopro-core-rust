//! Revelation of Hope
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 42104806;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Rank 8 Dragon Xyz Monster from your Extra Deck.
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
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.costfilter(c,e,tp)
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(8) and c:IsRace(RACE_DRAGON) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    if !(card::is_level(d, c, &[8])
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_faceup(d, c)
        && card::is_able_to_grave_as_cost(d, c))
    {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,costc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, costc: Option<CardId>) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,c,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, Some(c), None, REASON_XYZ);
    // return #pg<=0 and c:IsRank(8) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_XYZ)
    //     and Duel.GetLocationCountFromEx(tp,tp,costc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    if !(pg.is_empty() && card::is_rank(d, c, &[8]) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let count = match costc {
        Some(x) => duel::get_location_count_from_ex(d, tp, Some(tp), x.into(), duel::ExtraCard::Card(c), None).0,
        None => duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0,
    };
    count > 0 && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| costfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==1
        let cost_chk = effect::get_label(d, a.e) == 1;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk or Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
        return cost_chk || duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
