//! Duck Fighter
//! ダックファイター
use super::prelude::*;

pub const ID: u32 = 54813225;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcheck(sg,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8) -> bool {
    // return aux.ReleaseCheckMMZ(sg,tp) and sg:CheckWithSumGreater(Card.GetLevel,3,1,99)
    aux::release_check_mmz(d, sg, tp, &CardSet::new()).0
        && group::check_with_sum_greater(d, sg, &|d, x| card::get_level(d, x) as i64, 3).0
}
/// `s.spcheck` as the `check` argument of the release-cost functions
fn spcheck_rc(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> (bool, bool) {
    (spcheck(d, sg, tp), false)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsType,1,99,false,s.spcheck,nil,TYPE_TOKEN) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, tp, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), 1, 99, false, Some(&spcheck_rc), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsType,1,99,false,s.spcheck,nil,TYPE_TOKEN)
    let g = aux::select_release_group_cost(d, tp, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), 1, 99, false, Some(&spcheck_rc), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler()  (the card is looked up where the Lua uses it)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
