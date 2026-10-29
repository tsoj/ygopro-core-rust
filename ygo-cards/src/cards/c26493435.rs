//! Onomatopia
//! 希望郷－オノマトピア－
use super::prelude::*;

pub const ID: u32 = 26493435;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_UTOPIA, SET_ZUBABA, SET_GAGAGA, SET_GOGOGO, SET_DODODO],
    counter_place_list: &[COUNTER_HI_FIVE as u32],
    ..ScriptInfo::DEFAULT
};

const COUNTER_HI_FIVE: u16 = 0x30;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_HI_FIVE)
    card::enable_counter_permit(d, c, COUNTER_HI_FIVE, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 Hi-Five the Sky Counter on this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Monsters you control gain 200 ATK and DEF for each Hi-Five the Sky Counter on this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetValue(s.val)
    effect::set_value_fn(d, e3, effect_value(val));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Special Summon 1 "Zubaba", "Gagaga", "Gogogo", or "Dododo" monster from your Deck
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCost(s.spcost)
    effect::set_cost(d, e5, Some(cost_target(spcost)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.ctfilter(c,tp)
fn ctfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_UTOPIA) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UTOPIA as u16]) && card::is_controler(d, c, tp)
}
// function s.ctcon(e,tp,eg,ep,ev,re,r,rp)
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.ctfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d: &mut Duel, c| ctfilter(d, c, tp), 1, NO_EX)
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(COUNTER_HI_FIVE,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, COUNTER_HI_FIVE, 1, false);
}
// function s.val(e,c)
fn val(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetCounter(COUNTER_HI_FIVE)*200
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, COUNTER_HI_FIVE) as i64 * 200
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()  (the card is looked up where the Lua uses it)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,COUNTER_HI_FIVE,2,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, tp, COUNTER_HI_FIVE, 2, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,COUNTER_HI_FIVE,2,REASON_COST)
    card::remove_counter(d, c, tp, COUNTER_HI_FIVE, 2, REASON_COST);
    true
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_ZUBABA,SET_GAGAGA,SET_GOGOGO,SET_DODODO}) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ZUBABA as u16, SET_GAGAGA as u16, SET_GOGOGO as u16, SET_DODODO as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d: &mut Duel, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
