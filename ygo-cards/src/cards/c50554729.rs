//! Amorphage Infection
use super::prelude::*;

pub const ID: u32 = 50554729;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMORPHAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_AMORPHAGE))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_AMORPHAGE as u16]))));
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // To Hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetCode(EVENT_RELEASE)
    effect::set_code(d, e4, EVENT_RELEASE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.thcon1)
    effect::set_condition(d, e4, Some(condition(thcon1)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e5, EVENT_DESTROYED);
    // e5:SetCondition(s.thcon2)
    effect::set_condition(d, e5, Some(condition(thcon2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AMORPHAGE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AMORPHAGE as u16])
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,0,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)*100
    duel::get_matching_group_count(d, Some(&|d, c| filter(d, c)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64 * 100
}
/// `s.cfilter1(c,tp)`
fn cfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsPreviousLocation(LOCATION_MZONE|LOCATION_HAND) and c:IsPreviousControler(tp)
    card::is_monster(d, c) && card::is_previous_location(d, c, LOCATION_MZONE | LOCATION_HAND) && card::is_previous_controler(d, c, tp)
}
/// `s.thcon1(e,tp,eg,ep,ev,re,r,rp)`
fn thcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter1,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter1(d, c, a.tp), 1, NO_EX)
}
/// `s.cfilter2(c,tp)`
fn cfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and c:IsPreviousLocation(LOCATION_MZONE|LOCATION_HAND) and c:IsPreviousControler(tp)
    card::is_monster(d, c) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_MZONE | LOCATION_HAND) && card::is_previous_controler(d, c, tp)
}
/// `s.thcon2(e,tp,eg,ep,ev,re,r,rp)`
fn thcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter2,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter2(d, c, a.tp), 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_AMORPHAGE) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_AMORPHAGE as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
