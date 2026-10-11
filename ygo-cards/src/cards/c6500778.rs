//! Chaos Distill
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 6500778;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MACRO_COSMOS, 6500778], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Any card sent to your GY is banished instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_IGNORE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE | EFFECT_FLAG_IGNORE_IMMUNE | EFFECT_FLAG_IGNORE_RANGE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_ALL,0)
    effect::set_target_range(d, e1, LOCATION_ALL, 0);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // e1:SetTarget(function(e,c) return c:IsOwner(e:GetHandlerPlayer()) and Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),c) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| {
        let p = effect::get_handler_player(d, e);
        card::is_owner(d, c, p) && duel::is_player_can_remove(d, p, Some((c, None)))
    })));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND | CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.macrofilter(c)`: `return (c:IsCode(CARD_MACRO_COSMOS) or c:ListsCode(CARD_MACRO_COSMOS)) and not c:IsCode(id) and c:IsAbleToHand()`
fn macrofilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_code(d, c, &[CARD_MACRO_COSMOS]) || card::lists_code(d, c, &[CARD_MACRO_COSMOS])) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.monsterfilter(c)`: `return c:IsAttribute(ATTRIBUTE_LIGHT) and c:GetTextAttack()==-2 and c:IsAbleToHand()`
fn monsterfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::get_text_attack(d, c) == -2 && card::is_able_to_hand(d, c, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(s.macrofilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&macrofilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b2=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_MACRO_COSMOS),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //   and c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.monsterfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_MACRO_COSMOS]))),
        tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        NO_EX,
    ) && card::is_able_to_remove_as_cost(d, c, None)
        && duel::is_existing_matching_card(d, Some(&monsterfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==2 then
    if op == 2 {
        // Duel.Remove(c,POS_FACEUP,REASON_COST)
        duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local thfilter=(e:GetLabel()==1 and s.macrofilter or s.monsterfilter)
    let is_macro = effect::get_label(d, a.e) == 1;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = if is_macro {
        duel::select_matching_card(d, tp, Some(&macrofilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX)
    } else {
        duel::select_matching_card(d, tp, Some(&monsterfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX)
    }
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
