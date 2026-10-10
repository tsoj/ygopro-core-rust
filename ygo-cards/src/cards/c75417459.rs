//! Release Restraint
use super::prelude::*;

pub const ID: u32 = 75417459;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[423705, 57046845], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Gearfried the Swordmaster" from your hand or Deck
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
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsCode,1,false,aux.ReleaseCheckMMZ,nil,423705) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| card::is_code(d, x, &[423705])), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsCode,1,1,false,aux.ReleaseCheckMMZ,nil,423705)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, x| card::is_code(d, x, &[423705])), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(57046845) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[57046845]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==-100
        let cost_chk = effect::get_label(d, a.e) == -100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return (cost_chk or Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
        return (cost_chk || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
