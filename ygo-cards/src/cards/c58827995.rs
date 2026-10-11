//! Armor Dragon Ritual
use super::prelude::*;

pub const ID: u32 = 58827995;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75901113], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,4,nil,75901113)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(4)), None, &[75901113]);
    // Special summon 1 "Paladin of Armored Dragon" from GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetCondition(aux.exccon)
    effect::set_condition(d, e1, Some(aux::exccon()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetSum(Card.GetLevel)>=4
    (group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64) >= 4, false)
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:GetOriginalLevel()>0 and c:IsMonster() and c:IsAbleToRemoveAsCost()
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::get_original_level(d, c) > 0
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
/// Banish monsters whose total levels equal 4 as cost
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local rg=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,1,99,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(99), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local sg=aux.SelectUnselectGroup(rg,e,tp,1,99,s.rescon,1,tp,HINTMSG_REMOVE,nil,s.rescon)
    let sg = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(99), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, Some(&rescon), false)
        .unwrap_or_default();
    // Duel.Remove(sg,POS_FACEUP,REASON_COST)
    duel::remove(d, &sg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(75901113) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[75901113]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsAbleToRemove() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
/// Banish itself from GY, and if you do, special summon 1 "Paladin of Armored Dragon" from GY
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.Remove(c,POS_FACEUP,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, c, e) && duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
