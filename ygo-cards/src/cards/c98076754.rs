//! Chosen of Zefra
use super::prelude::*;

pub const ID: u32 = 98076754;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZEFRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // e2:SetLabel(3)
    effect::set_label(d, e2, &[3]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent destruction by opponent's effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetValue(aux.indoval)
    effect::set_value_fn(d, e3, aux::indoval());
    // e3:SetLabel(5)
    effect::set_label(d, e3, &[5]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Prevent effect target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetCondition(s.effcon)
    effect::set_condition(d, e4, Some(condition(effcon)));
    // e4:SetValue(aux.tgoval)
    effect::set_value_fn(d, e4, aux::tgoval());
    // e4:SetLabel(8)
    effect::set_label(d, e4, &[8]);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Shuffle all cards into the Deck
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e5, CATEGORY_TODECK);
    // e5:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    // e5:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCondition(s.tdcon)
    effect::set_condition(d, e5, Some(condition(tdcon)));
    // e5:SetCost(s.tdcost)
    effect::set_cost(d, e5, Some(cost_target(tdcost)));
    // e5:SetTarget(s.tdtg)
    effect::set_target(d, e5, Some(cost_target(tdtg)));
    // e5:SetOperation(s.tdop)
    effect::set_operation(d, e5, Some(operation(tdop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_ZEFRA),e:GetHandlerPlayer(),LOCATION_EXTRA,0,nil):GetClassCount(Card.GetCode)>=e:GetLabel()
    let hp = effect::get_handler_player(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| zefra_faceup(d, c)), hp, LOCATION_EXTRA, 0, NO_EX);
    let count = group::get_class_count(d, &g, &code_value) as i64;
    count >= effect::get_label(d, a.e)
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_ZEFRA)`
fn zefra_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ZEFRA as u16])
}
/// `Card.GetCode` as the class of `GetClassCount` (its first code)
fn code_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_code(d, c)[0] as i64
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsFaceup,0,LOCATION_EXTRA,LOCATION_EXTRA,nil)*100
    duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_EXTRA, LOCATION_EXTRA, NO_EX) as i64 * 100
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_ZEFRA),tp,LOCATION_EXTRA,0,nil):GetClassCount(Card.GetCode)==10
    let g = duel::get_matching_group(d, Some(&|d, c| zefra_faceup(d, c)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    group::get_class_count(d, &g, &code_value) == 10
}
/// `s.tdcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let zone = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, 0, zone, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), a.tp, 0, zone, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(Card.IsAbleToDeck),tp,0,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let f = aux::necro_valley_filter(card::is_able_to_deck);
    let g = duel::get_matching_group(d, Some(&f), a.tp, 0, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
