//! King's Sarcophagus
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 16528181;

// s.listed_series={SET_HORUS}
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HORUS],
    ..ScriptInfo::DEFAULT
};

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
    // Your "Horus" monsters cannot be destroyed by effects that do not target them
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_HORUS))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_HORUS as u16]))));
    // e2:SetValue(s.indvalue)
    effect::set_value_fn(d, e2, lua_fn(indvalue));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 "Horus" monster from the Deck to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(4,id)
    effect::set_count_limit(d, e3, 4, ID, 0, 0);
    // e3:SetCost(s.dtgcost)
    effect::set_cost(d, e3, Some(cost_target(dtgcost)));
    // e3:SetTarget(s.dtgtg)
    effect::set_target(d, e3, Some(cost_target(dtgtg)));
    // e3:SetOperation(s.dtgop)
    effect::set_operation(d, e3, Some(operation(dtgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Send to the GY an opponent's monster that battles your "Horus" monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e4, EVENT_BATTLE_START);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.btgcond)
    effect::set_condition(d, e4, Some(condition(btgcond)));
    // e4:SetTarget(s.btgtg)
    effect::set_target(d, e4, Some(cost_target(btgtg)));
    // e4:SetOperation(s.btgop)
    effect::set_operation(d, e4, Some(operation(btgop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.indvalue(e,re,rp,c)`
fn indvalue(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let c = a.c(3);
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return true end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return true;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // return not g:IsContains(c)
    !group::is_contains(&g, c)
}
/// `s.dtgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dtgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,1,1,REASON_COST,nil)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), 1, 1, REASON_COST, NO_EX);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_HORUS) and c:IsMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_HORUS as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.dtgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.dtgop(e,tp,eg,ep,ev,re,r,rp)`
fn dtgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.btgcond(e,tp,eg,ep,ev,re,r,rp)`
fn btgcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,at=Duel.GetBattleMonster(tp)
    let (x, at) = duel::get_battle_monster(d, a.tp);
    // return a and at and a:IsSetCard(SET_HORUS) and a:IsFaceup()
    let Some(x) = x else { return false };
    at.is_some() && card::is_set_card(d, x, &[SET_HORUS as u16]) && card::is_faceup(d, x)
}
/// `s.btgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn btgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,at=Duel.GetBattleMonster(tp)
    let (_x, at) = duel::get_battle_monster(d, a.tp);
    // if chk==0 then return at and at:IsAbleToGrave() end
    if a.chk == Some(0) {
        return at.is_some_and(|at| card::is_able_to_grave(d, at, None));
    }
    // e:SetLabelObject(at)
    effect::set_label_object(d, a.e, at.map_or(Value::Nil, Value::Card));
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,at,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, at.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.btgop(e,tp,eg,ep,ev,re,r,rp)`
fn btgop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e);
    // if tc and tc:IsRelateToBattle() and tc:IsControler(1-tp) then
    if let Some(tc) = tc.as_card() {
        if card::is_relate_to_battle(d, tc) && card::is_controler(d, tc, 1 - a.tp) {
            // Duel.SendtoGrave(tc,REASON_EFFECT)
            duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
        }
    }
}
