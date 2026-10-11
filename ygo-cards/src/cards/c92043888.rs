//! Solfachord Formal
use super::prelude::*;

pub const ID: u32 = 92043888;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    //     and Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_PZONE,0,1,nil,SET_SOLFACHORD)
    if a.rp != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    let re_ok = effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE);
    re_ok && duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_SOLFACHORD as u16])), a.tp, LOCATION_PZONE, 0, 1, NO_EX)
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.disfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| disfilter(d, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.disfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| disfilter(d, c)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and Duel.GetOperatedGroup():GetFirst():IsLocation(LOCATION_DECK) then
    if duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        let operated = duel::get_operated_group(d);
        let first = group::get_first(d, &operated).nil();
        if card::is_location(d, first, LOCATION_DECK) {
            // Unaffected
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
            // e1:SetTargetRange(LOCATION_MZONE,0)
            effect::set_target_range(d, e1, LOCATION_MZONE, 0);
            // e1:SetTarget(aux.TargetBoolFunction(Card.IsType,TYPE_PENDULUM))
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_PENDULUM))));
            // e1:SetValue(s.efilter)
            effect::set_value_fn(d, e1, lua_fn(efilter));
            // e1:SetLabelObject(re)
            effect::set_label_object(d, e1, label_of(a.re));
            // e1:SetReset(RESET_CHAIN)
            effect::set_reset(d, e1, RESET_CHAIN, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
            // Cannot be destroyed
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD);
            // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
            // e2:SetTargetRange(LOCATION_PZONE,0)
            effect::set_target_range(d, e2, LOCATION_PZONE, 0);
            // e2:SetValue(s.efilter)
            effect::set_value_fn(d, e2, lua_fn(efilter));
            // e2:SetLabelObject(re)
            effect::set_label_object(d, e2, label_of(a.re));
            // e2:SetReset(RESET_CHAIN)
            effect::set_reset(d, e2, RESET_CHAIN, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
            // Cannot be banished
            // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_PZONE,0,nil)
            let g = duel::get_matching_group(d, None, tp, LOCATION_PZONE, 0, NO_EX);
            // for tc in aux.Next(g) do
            for tc in group::iter(d, &g) {
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e3, EFFECT_TYPE_FIELD);
                // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
                effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
                // e3:SetCode(EFFECT_CANNOT_REMOVE)
                effect::set_code(d, e3, EFFECT_CANNOT_REMOVE);
                // e3:SetRange(LOCATION_PZONE)
                effect::set_range(d, e3, LOCATION_PZONE);
                // e3:SetTargetRange(1,1)
                effect::set_target_range(d, e3, 1, 1);
                // e3:SetTarget(s.rmlimit)
                effect::set_target(d, e3, Some(lua_fn(rmlimit)));
                // e3:SetLabelObject(re)
                effect::set_label_object(d, e3, label_of(a.re));
                // e3:SetReset(RESET_CHAIN)
                effect::set_reset(d, e3, RESET_CHAIN, 1);
                // tc:RegisterEffect(e3)
                card::register_effect(d, tc, e3, false);
            }
        }
    }
}
/// The `re` of the activation as a label object (`nil` when the Lua `re` is nil).
fn label_of(re: Option<EffectId>) -> Value {
    match re {
        Some(r) => Value::Effect(r),
        None => Value::Nil,
    }
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return re==e:GetLabelObject()
    let lo = effect::get_label_object(d, a.e(0)).as_effect();
    lo == a.v(1).as_effect()
}
/// `s.rmlimit(e,c,tp,r,re)`
fn rmlimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsLocation(LOCATION_PZONE) and re==e:GetLabelObject()
    let c = a.c(1);
    if !card::is_location(d, c, LOCATION_PZONE) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e(0)).as_effect();
    lo == a.v(4).as_effect()
}
