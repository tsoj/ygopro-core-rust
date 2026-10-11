//! Dances with Beasts
use super::prelude::*;

pub const ID: u32 = 33298291;

const LOCATION_HDG: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil and g:GetSum(Card.GetAttack)>=8000
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
        && duel::get_attack_target(d).is_none()
        && group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64) >= 8000
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
/// `s.spcheck(sg,e,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetOriginalCode)==#sg and sg:GetClassCount(Card.GetLocation)==#sg
    let n = sg.len();
    let res = group::get_class_count(d, sg, &|d, c| card::get_original_code(d, c) as i64) == n
        && group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) == n;
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HDG,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HDG, 0, NO_EX);
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>2
        //   and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //   and aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,0)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&spcheck), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,LOCATION_HDG)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, a.tp, LOCATION_HDG as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<3 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 3
        || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HDG,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_HDG, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&spcheck), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==3 then
    if sg.len() == 3 {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
}
