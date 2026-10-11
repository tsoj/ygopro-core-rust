//! Gateway of the Six
use super::prelude::*;

pub const ID: u32 = 27970830;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_BUSHIDO], listed_series: &[SET_SIX_SAMURAI, SET_SHIEN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_BUSHIDO)
    card::enable_counter_permit(d, c, COUNTER_BUSHIDO as u16, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atkup
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCost(s.cost1)
    effect::set_cost(d, e4, Some(cost_target(cost1)));
    // e4:SetTarget(s.tg1)
    effect::set_target(d, e4, Some(cost_target(tg1)));
    // e4:SetOperation(s.op1)
    effect::set_operation(d, e4, Some(operation(op1)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // search
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCost(s.cost2)
    effect::set_cost(d, e5, Some(cost_target(cost2)));
    // e5:SetTarget(s.tg2)
    effect::set_target(d, e5, Some(cost_target(tg2)));
    // e5:SetOperation(s.op2)
    effect::set_operation(d, e5, Some(operation(op2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // special summon
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_CARD_TARGET, 0);
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCost(s.cost3)
    effect::set_cost(d, e6, Some(cost_target(cost3)));
    // e6:SetTarget(s.tg3)
    effect::set_target(d, e6, Some(cost_target(tg3)));
    // e6:SetOperation(s.op3)
    effect::set_operation(d, e6, Some(operation(op3)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(aux.FaceupFilter(Card.IsSetCard,SET_SIX_SAMURAI),1,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]), 1, NO_EX) {
        // e:GetHandler():AddCounter(COUNTER_BUSHIDO,2)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, COUNTER_BUSHIDO as u16, 2, false);
    }
}
/// Shared body of `s.cost1` / `s.cost2` / `s.cost3`: remove `n` Bushido Counters as a cost.
fn bushido_cost(d: &mut Duel, a: &EffArgs, n: u32) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_BUSHIDO,n,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_BUSHIDO as u16, n, REASON_COST);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.RemoveCounter(tp,1,0,COUNTER_BUSHIDO,n,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_BUSHIDO as u16, n, REASON_COST);
    true
}
/// `s.cost1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost1(d: &mut Duel, a: &EffArgs) -> bool {
    bushido_cost(d, a, 2)
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_SIX_SAMURAI,SET_SHIEN}) and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16, SET_SHIEN as u16]) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.tg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,0,500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g, 1, 0, 500);
    true
}
/// `s.op1(e,tp,eg,ep,ev,re,r,rp)`
fn op1(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.cost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    bushido_cost(d, a, 4)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_SIX_SAMURAI) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.tg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter2(d, c)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.op2(e,tp,eg,ep,ev,re,r,rp)`
fn op2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter2),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nf = aux::necro_valley_filter(|d: &mut Duel, c: CardId| filter2(d, c));
    let g = duel::select_matching_card(d, a.tp, Some(&nf), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.cost3(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost3(d: &mut Duel, a: &EffArgs) -> bool {
    bushido_cost(d, a, 6)
}
/// `s.filter3(c,e,tp)`
fn filter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SHIEN) and c:IsType(TYPE_EFFECT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SHIEN as u16])
        && card::is_type(d, c, TYPE_EFFECT)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tg3(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tg3(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter3(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter3(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.filter3,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter3(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter3,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter3(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.op3(e,tp,eg,ep,ev,re,r,rp)`
fn op3(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
