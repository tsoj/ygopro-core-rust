//! Emblema Salvation
use super::prelude::*;

pub const ID: u32 = 61950680;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CENTUR_ION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 card your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCost(s.rmcost)
    effect::set_cost(d, e1, Some(cost_target(rmcost)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 2 "Centur-Ion" monsters in the Spell/Trap Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.plcon)
    effect::set_condition(d, e2, Some(condition(plcon)));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_REMOVE)
    effect::set_code(d, e3, EVENT_REMOVE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rmcostfilter(c)`
fn rmcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CENTUR_ION) and c:IsMonsterCard() and c:IsFaceup() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_CENTUR_ION as u16]) && card::is_monster_card(d, c) && card::is_faceup(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmcostfilter,tp,LOCATION_STZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmcostfilter), tp, LOCATION_STZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmcostfilter,tp,LOCATION_STZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rmcostfilter), tp, LOCATION_STZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.plcon(e,tp,eg,ep,ev,re,r,rp)`
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_EFFECT) and re and re:IsActivated() and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    if a.rp != 1 - tp || !card::is_reason(d, c, REASON_EFFECT) {
        return false;
    }
    let Some(re) = a.re else { return false };
    effect::is_activated(d, re)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEDOWN)
}
/// `s.plfilter(c)`
fn plfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CENTUR_ION) and c:IsMonster() and not c:IsForbidden()
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_HAND|LOCATION_DECK))
    card::is_set_card(d, c, &[SET_CENTUR_ION as u16])
        && card::is_monster(d, c)
        && !card::is_forbidden(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND | LOCATION_DECK))
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<2 then return false end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 2 {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.plfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
        let g = duel::get_matching_group(d, Some(&plfilter2), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0)
        let rs = aux::dncheck();
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rs), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.plfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| plfilter2(d, c))), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_TOFIELD)
    let rs = aux::dncheck();
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rs), 1, tp, Some(HINTMSG_TOFIELD as u64), None, None, false).unwrap_or_default();
    // if #tg~=2 then return end
    if tg.len() != 2 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // if Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
        if duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // Treat as Continuous Trap
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_TRAP|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_TRAP | TYPE_CONTINUOUS) as i64);
            // e1:SetReset((RESET_EVENT|RESETS_STANDARD)&~RESET_TURN_SET)
            effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) & !RESET_TURN_SET, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
