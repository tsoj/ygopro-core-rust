//! Goddess of Sweet Revenge
use super::prelude::*;

pub const ID: u32 = 72589042;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy opponent's cards and Special summon from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetMatchingGroupCount(nil,tp,LOCATION_ONFIELD|LOCATION_HAND,0,e:GetHandler())==0
    let attacker = duel::get_attacker(d).nil();
    let handler = effect::get_handler(d, a.e).nil();
    card::is_controler(d, attacker, 1 - a.tp)
        && duel::get_matching_group_count(d, None, a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0, handler.into()) == 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #sg>0 end
    if a.chk == Some(0) {
        return !sg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #sg>0 and Duel.Destroy(sg,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !sg.is_empty()
        && duel::destroy(d, sg, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let tc = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
