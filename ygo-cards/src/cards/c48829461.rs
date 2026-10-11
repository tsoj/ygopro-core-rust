//! Malefic Paradox Gear
use super::prelude::*;

pub const ID: u32 = 48829461;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], listed_names: &[74509280], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    // e1:SetDescription(aux.Stringid(id,0))
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SEARCH+CATEGORY_TOHAND)
    // e1:SetType(EFFECT_TYPE_IGNITION)
    // e1:SetRange(LOCATION_MZONE)
    // e1:SetCountLimit(1,id)
    // e1:SetCondition(s.spcon)
    // e1:SetCost(Cost.SelfTribute)
    // e1:SetTarget(s.sptg)
    // e1:SetOperation(s.spop)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // remove replace
    // local e2=Effect.CreateEffect(c)
    // e2:SetType(EFFECT_TYPE_SINGLE)
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    // e2:SetRange(LOCATION_GRAVE|LOCATION_MZONE)
    // e2:SetCode(id)
    // e2:SetCountLimit(1,{id,1})
    // c:RegisterEffect(e2)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_GRAVE | LOCATION_MZONE);
    effect::set_code(d, e2, ID);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    let _ = a;
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
// function s.spfilter(c,e,tp,chk)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, chk: bool) -> bool {
    // return c:IsCode(74509280) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (not chk or Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c))
    card::is_code(d, c, &[74509280])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (!chk || duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, c.into()))
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>-1 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,true) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, true)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler(c)
    let _c = effect::get_handler(d, e);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, false)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
        if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local ag=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
            let ag = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #ag>0 then
            if !ag.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(ag,tp,REASON_EFFECT)
                duel::send_to_hand(d, ag.clone(), Some(tp), REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,ag)
                duel::confirm_cards(d, 1 - tp, ag);
            }
        }
    }
}
