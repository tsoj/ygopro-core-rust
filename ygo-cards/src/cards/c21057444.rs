//! Marincess Springirl
use super::prelude::*;

pub const ID: u32 = 21057444;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

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
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
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
    // Send cards from the top of your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DECKDES+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DECKDES + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.mlcon)
    effect::set_condition(d, e2, Some(condition(mlcon)));
    // e2:SetTarget(s.mltg)
    effect::set_target(d, e2, Some(cost_target(mltg)));
    // e2:SetOperation(s.mlop)
    effect::set_operation(d, e2, Some(operation(mlop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcostfilter(c)
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MARINCESS) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_MARINCESS as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&spcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
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
// function s.mlcon(e,tp,eg,ep,ev,re,r,rp)
fn mlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_GRAVE) and r==REASON_LINK and c:GetReasonCard():IsAttribute(ATTRIBUTE_WATER)
    card::is_location(d, c, LOCATION_GRAVE) && a.r == REASON_LINK && {
        let rc = card::get_reason_card(d, c).nil();
        card::is_attribute(d, rc, ATTRIBUTE_WATER)
    }
}
// function s.mltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mltg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_MARINCESS),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16])), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDiscardDeck(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_discard_deck(d, a.tp, ct as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, ct as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,200)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 200);
    true
}
// function s.mlop(e,tp,eg,ep,ev,re,r,rp)
fn mlop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_MARINCESS),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MARINCESS as u16])), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if ct<1 or Duel.DiscardDeck(tp,ct,REASON_EFFECT)<1 then return end
    if ct < 1 || duel::discard_deck(d, a.tp, ct as u16, REASON_EFFECT) < 1 {
        return;
    }
    // local dc=Duel.GetOperatedGroup():Match(Card.IsSetCard,nil,SET_MARINCESS):Match(Card.IsLocation,nil,LOCATION_GRAVE):GetCount()
    let mut g = duel::get_operated_group(d);
    group::match_(d, &mut g, &|d, c| card::is_set_card(d, c, &[SET_MARINCESS as u16]), NO_EX);
    group::match_(d, &mut g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    let dc = g.len() as i64;
    // if dc>0 then
    if dc > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,dc*200,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, dc * 200, REASON_EFFECT, false, None);
    }
}
