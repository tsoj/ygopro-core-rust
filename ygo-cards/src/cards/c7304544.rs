//! Naturia Pineapple
use super::prelude::*;

pub const ID: u32 = 7304544;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change race
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e1, EFFECT_CHANGE_RACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetValue(RACE_PLANT)
    effect::set_value(d, e1, RACE_PLANT as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() or (c:IsCode(id) and c:IsFaceup())
    card::is_spell_trap(d, c) || (card::is_code(d, c, &[ID]) && card::is_faceup(d, c))
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and not c:IsRace(RACE_PLANT|RACE_BEAST)
    card::is_monster(d, c) && !card::is_race(d, c, (RACE_PLANT | RACE_BEAST) as u64)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return tp==Duel.GetTurnPlayer() and not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_GRAVE,0,1,nil)
    tp == duel::get_turn_player(d)
        && !duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if e:GetHandler():IsRelateToEffect(e)
    let h = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, h, e)
        // and not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,0,1,nil)
        && !duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        // and not Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_GRAVE,0,1,nil) then
        && !duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_GRAVE, 0, 1, NO_EX)
    {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, h, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
