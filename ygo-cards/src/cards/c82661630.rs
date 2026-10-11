//! Ryu-Ge Realm - Dino Domains
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 82661630;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYU_GE], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can only control 1 "Ryu-Ge Realm - Dino Domains"
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // "Ryu-Ge" monsters you control gain 300 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_RYU_GE))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_RYU_GE as u16]))));
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of a monster effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetCost(s.negcost)
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev) Duel.NegateActivation(ev) end)
    effect::set_operation(d, e2, Some(lua_fn(negop_act)));
    // Grant the above effect to "Ryu-Ge" Pendulum Monsters and Level 10 or higher monsters whose original Type is Dinosaur you control
    // local e3=Effect.CreateEffect(c)
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
    // Monsters affected by this card's effect become Effect Monsters
    // local e4=Effect.CreateEffect(c)
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
/// `s.efftg(e,c)`
fn efftg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return (c:IsSetCard(SET_RYU_GE) and c:IsType(TYPE_PENDULUM)) or (c:IsLevelAbove(10) and c:IsOriginalRace(RACE_DINOSAUR))
    (card::is_set_card(d, c, &[SET_RYU_GE as u16]) && card::is_type(d, c, TYPE_PENDULUM))
        || (card::is_level_above(d, c, 10) && card::is_original_race(d, c, RACE_DINOSAUR))
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local trig_loc,trig_typ,trig_atk=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_TYPE,CHAININFO_TRIGGERING_ATTACK)
    let info = duel::get_chain_info(
        d,
        0,
        &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_TYPE, CHAININFO_TRIGGERING_ATTACK],
    );
    let trig_loc = info[0].int();
    let trig_typ = info[1].int();
    let trig_atk = info[2].int();
    // return trig_loc&LOCATION_MZONE>0 and trig_typ&TYPE_MONSTER>0 and e:GetHandler():GetAttack()>trig_atk
    let h = effect::get_handler(d, a.e).nil();
    (trig_loc & LOCATION_MZONE as i64) > 0
        && (trig_typ & TYPE_MONSTER as i64) > 0
        && (card::get_attack(d, h) as i64) > trig_atk
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYU_GE) and c:IsContinuousSpell() and c:IsFaceup() and c:IsAbleToDeckAsCost()
    card::is_set_card(d, c, &[SET_RYU_GE as u16])
        && card::is_continuous_spell(d, c)
        && card::is_faceup(d, c)
        && card::is_able_to_deck_as_cost(d, c)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
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
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `function(e,tp,eg,ep,ev) Duel.NegateActivation(ev) end`
fn negop_act(d: &mut Duel, a: Args) {
    duel::negate_activation(d, a.int(4) as u8);
}
