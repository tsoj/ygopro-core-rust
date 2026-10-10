//! Shiranui Style Solemnity
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 40364916;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[40005099], ..ScriptInfo::DEFAULT };

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
    // Change name to "Shiranui Style Synthesis"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e2, EFFECT_CHANGE_CODE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetValue(40005099)
    effect::set_value(d, e2, 40005099);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent the negation of summons
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e3:SetCost(s.ndisabcost)
    effect::set_cost(d, e3, Some(cost_target(ndisabcost)));
    // e3:SetOperation(s.ndisabop)
    effect::set_operation(d, e3, Some(operation(ndisabop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Banish 1 monster and send to the GY 1 Zombie monster from the Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_REMOVE+CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_REMOVE + CATEGORY_TOGRAVE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e4, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e4:SetTarget(s.rmvtg)
    effect::set_target(d, e4, Some(cost_target(rmvtg)));
    // e4:SetOperation(s.rmvop)
    effect::set_operation(d, e4, Some(operation(rmvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.rmvfilter1(c)`
fn rmvfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsAbleToRemoveAsCost()
    card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.ndisabcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ndisabcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmvfilter1,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmvfilter1), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmvfilter1,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rmvfilter1), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.ndisabop(e,tp,eg,ep,ev,re,r,rp)`
fn ndisabop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DISABLE_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_DISABLE_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_ZOMBIE))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_ZOMBIE))));
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
/// `s.rmvfilter(c)`
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_ZOMBIE) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.tgfilter(c,e,tp)` (the Lua passes no extra arguments to it, so the filter only sees `c`)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsDefenseBelow(0)
    card::is_race(d, c, RACE_ZOMBIE) && card::is_defense_below(d, c, 0)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rmvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rmvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmvfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmvfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmvfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&rmvfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, tc, e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0
        && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
