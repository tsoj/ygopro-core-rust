//! Ryu-Ge Realm - Wyrm Winds
use super::prelude::*;

pub const ID: u32 = 55154344;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_RYU_GE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can only control 1 "Ryu-Ge Realm - Wyrm Winds"
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Any monster sent from the field to the GY during your opponent's turn is banished instead
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(function(e) return Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(function(e,c) return Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),c) end)
    effect::set_target(d, e1, Some(effect_card_fn(redirect_target)));
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the ATK of 1 face-up monster to 0
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // Grant the above effect to "Ryu-Ge" Pendulum Monsters and Level 10 or higher monsters whose original Type is Wyrm you control
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(effect_card_fn(efftg)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Monsters affected by this card's effect become effect monsters
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e4, EFFECT_ADD_TYPE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetTarget(s.efftg)
    effect::set_target(d, e4, Some(effect_card_fn(efftg)));
    // e4:SetValue(TYPE_EFFECT)
    effect::set_value(d, e4, TYPE_EFFECT as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) return Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end` (e1's condition)
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_turn_player(d, 1 - p)
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end` (e2's condition)
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `function(e,c) return Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),c) end` (e1's target)
fn redirect_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    duel::is_player_can_remove(d, p, Some((c, None)))
}
/// `s.efftg(e,c)`: `return (c:IsSetCard(SET_RYU_GE) and c:IsType(TYPE_PENDULUM)) or (c:IsLevelAbove(10) and c:IsOriginalRace(RACE_WYRM))`
fn efftg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    (card::is_set_card(d, c, &[SET_RYU_GE as u16]) && card::is_type(d, c, TYPE_PENDULUM))
        || (card::is_level_above(d, c, 10) && card::is_original_race(d, c, RACE_WYRM as u64))
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYU_GE) and c:IsContinuousSpell() and c:IsFaceup() and c:IsAbleToDeckAsCost()
    //   and Duel.IsExistingTarget(Card.HasNonZeroAttack,0,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_set_card(d, c, &[SET_RYU_GE as u16])
        && card::is_continuous_spell(d, c)
        && card::is_faceup(d, c)
        && card::is_able_to_deck_as_cost(d, c)
        && duel::is_existing_target(d, Some(&|d, x| card::has_non_zero_attack(d, x)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_COST, None);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:HasNonZeroAttack() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::has_non_zero_attack(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::has_non_zero_attack(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local tc=Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, x| card::has_non_zero_attack(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,-tc:GetAttack())
    let atk = card::get_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(tc)), 1, a.tp, -atk);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
