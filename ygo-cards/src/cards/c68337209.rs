//! Maliss in Underground
use super::prelude::*;

pub const ID: u32 = 68337209;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALISS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // While you have 3 or more banished "Maliss" Traps with different names, "Maliss" Link Monsters you control gain 3000 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.atkupcon)
    effect::set_condition(d, e2, Some(condition(atkupcon)));
    // e2:SetTarget(function(e,c) return s.atkfilter(c) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| atkfilter(d, c))));
    // e2:SetValue(3000)
    effect::set_value(d, e2, 3000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While you control any "Maliss" Link Monsters, your opponent's monsters can only target those monsters for attacks
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.atktgcon)
    effect::set_condition(d, e3, Some(condition(atktgcon)));
    // e3:SetValue(function(e,c) return not s.atkfilter(c) end)
    effect::set_value_fn(d, e3, effect_card_fn(|d, _e, c| !atkfilter(d, c)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALISS) and c:IsAbleToRemove()
    card::is_set_card(d, c, &[SET_MALISS as u16]) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.rmfilter),tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,nil)
    let f = aux::necro_valley_filter(|d, c| rmfilter(d, c));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALISS) and c:IsType(TYPE_LINK) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_MALISS as u16]) && card::is_type(d, c, TYPE_LINK) && card::is_faceup(d, c)
}
/// `s.atkupconfilter(c)`
fn atkupconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALISS) and c:IsTrap() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_MALISS as u16]) && card::is_trap(d, c) && card::is_faceup(d, c)
}
/// `s.atkupcon(e)`
fn atkupcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.atkupconfilter,e:GetHandlerPlayer(),LOCATION_REMOVED,0,nil)
    let p = effect::get_handler_player(d, a.e);
    let g = duel::get_matching_group(d, Some(&atkupconfilter), p, LOCATION_REMOVED, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>=3
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3
}
/// `s.atktgcon(e)`
fn atktgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.atkfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&atkfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
