//! Ixeep, Omen of the Ghoti
use super::prelude::*;

pub const ID: u32 = 3137279;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOTI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_REMOVE)
    effect::set_code(d, e1, EVENT_REMOVE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e2, LOCATION_REMOVED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.setcond)
    effect::set_condition(d, e2, Some(condition(setcond)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcfilter(c)
fn spcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsMonster() and c:IsRace(RACE_FISH) and not c:IsType(TYPE_TOKEN)
    //   and (not c:IsPreviousLocation(LOCATION_ONFIELD) or c:GetPreviousRaceOnField()&RACE_FISH>0)
    card::is_faceup(d, c)
        && card::is_monster(d, c)
        && card::is_race(d, c, RACE_FISH)
        && !card::is_type(d, c, TYPE_TOKEN)
        && (!card::is_previous_location(d, c, LOCATION_ONFIELD) || (card::get_previous_race_on_field(d, c) & RACE_FISH) > 0)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spcfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &spcfilter, 1, NO_EX)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.setcond(e,tp,eg,ep,ev,re,r,rp)
fn setcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()==e:GetHandler():GetTurnID()+1
    let c = effect::get_handler(d, a.e).nil();
    let tid = card::get_turn_id(d, c) as i32;
    duel::get_turn_count(d, None) == tid + 1
}
// function s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsFaceup() and c:IsSetCard(SET_GHOTI) and c:IsSSetable()
    card::is_trap(d, c) && card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GHOTI as u16]) && card::is_s_setable(d, c, false, None)
}
// function s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and s.setfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && setfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&setfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&setfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if g:GetFirst():IsLocation(LOCATION_GRAVE) then
    let first = group::get_first(d, &g).nil();
    if card::is_location(d, first, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, a.tp, 0);
    }
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsSSetable() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) {
        // Duel.SSet(tp,tc)
        duel::s_set(d, a.tp, tc, None, true);
    }
}
