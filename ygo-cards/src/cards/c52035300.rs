//! The Immortal Bushi
use super::prelude::*;

pub const ID: u32 = 52035300;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // release limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.recon)
    effect::set_value_fn(d, e1, effect_card_fn(recon));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
    effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.recon(e,c)`
fn recon(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsRace(RACE_WARRIOR)
    !card::is_race(d, c, RACE_WARRIOR)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and not c:IsRace(RACE_WARRIOR)
    card::is_monster(d, c) && !card::is_race(d, c, RACE_WARRIOR)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    //   and not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil)
    a.tp == duel::get_turn_player(d)
        && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
        && !duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 or Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) > 0 {
        return;
    }
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, h, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
