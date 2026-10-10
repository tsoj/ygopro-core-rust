//! Hand of Nephthys
use super::prelude::*;

pub const ID: u32 = 98446407;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[61441708], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.mzfilter(c,tp)`
fn mzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:GetSequence()<5
    card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if c:GetSequence()<5 then ft=ft+1 end
    if card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>-1 and c:IsReleasable() and Duel.CheckReleaseGroupCost(tp,nil,1,false,nil,c)
    //     and (ft>0 or Duel.CheckReleaseGroupCost(tp,s.mzfilter,1,false,nil,c,tp)) end
    if a.chk == Some(0) {
        return ft > -1
            && card::is_releasable(d, c, None, None)
            && duel::check_release_group_cost(d, a.tp, None, 1, 1, false, None, c.into())
            && (ft > 0 || duel::check_release_group_cost(d, a.tp, Some(&|d, x| mzfilter(d, x, a.tp)), 1, 1, false, None, c.into()));
    }
    // local rg=nil
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // if ft>0 then
    let mut rg = if ft > 0 {
        // rg=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,nil,c)
        duel::select_release_group_cost(d, a.tp, None, 1, 1, false, None, c.into())
    } else {
        // rg=Duel.SelectReleaseGroupCost(tp,s.mzfilter,1,1,false,nil,c,tp)
        duel::select_release_group_cost(d, a.tp, Some(&|d, x| mzfilter(d, x, a.tp)), 1, 1, false, None, c.into())
    };
    // rg:AddCard(c)
    group::add_card(&mut rg, &CardSet::from_card(c));
    // Duel.Release(rg,REASON_COST)
    duel::release(d, &rg, REASON_COST, None);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(61441708) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[61441708]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
