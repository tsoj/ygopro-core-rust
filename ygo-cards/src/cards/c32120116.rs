//! Enma's Judgment
//! 閻魔の裁き
use super::prelude::*;

pub const ID: u32 = 32120116;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local eg = a.eg
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // local g=eg:Filter(Card.IsSummonPlayer,nil,1-tp)
    let g = group::filter(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsRelateToEffect(e)
    card::is_summon_player(d, c, tp) && card::is_relate_to_effect(d, c, e)
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsAbleToRemove()
    card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsLevelAbove(7) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_ZOMBIE)
        && card::is_level_above(d, c, 7)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=eg:Filter(s.filter2,nil,e,1-tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter2(d, c, e, 1 - tp), NO_EX);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local g1=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_GRAVE,0,nil)
        let g1 = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_GRAVE, 0, NO_EX);
        // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
        let g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // if #g1>4 and #g2>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if g1.len() > 4 && !g2.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // local rg=g1:Select(tp,5,5,nil)
            let rg = group::select(d, &g1, tp, 5, 5, false, NO_EX).nil();
            // if Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)==5 then
            if duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 5 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sg=g2:Select(tp,1,1,nil)
                let sg = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
                // if #sg>0 then
                if !sg.is_empty() {
                    // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
}
