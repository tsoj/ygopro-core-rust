//! Malefic Force
use super::prelude::*;

pub const ID: u32 = 68721020;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 face-up monster on the field; equip this card to it as an Equip Card with these effects
    // local e1=Effect.CreateEffect(c)
    // e1:SetDescription(aux.Stringid(id,0))
    // e1:SetCategory(CATEGORY_EQUIP)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // e1:SetCountLimit(1,{id,0})
    // e1:SetCost(aux.RemainFieldCost)
    // e1:SetTarget(s.eqtg)
    // e1:SetOperation(s.eqop)
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    card::register_effect(d, c, e1, false);
    // If a card is in the Field Zone: You can banish this card and 1 other "Malefic" card from your GY; destroy all monsters on the field
    // local e2=Effect.CreateEffect(c)
    // e2:SetDescription(aux.Stringid(id,1))
    // e2:SetCategory(CATEGORY_DESTROY)
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    // e2:SetCode(EVENT_FREE_CHAIN)
    // e2:SetRange(LOCATION_GRAVE)
    // e2:SetCountLimit(1,{id,1})
    // e2:SetCondition(function() return Duel.IsExistingMatchingCard(nil,0,LOCATION_FZONE,LOCATION_FZONE,1,nil) end)
    // e2:SetCost(s.descost)
    // e2:SetTarget(s.destg)
    // e2:SetOperation(s.desop)
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    // c:RegisterEffect(e2)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_cost(d, e2, Some(cost_target(descost)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    effect::set_hint_timing(d, e2, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    card::register_effect(d, c, e2, false);
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsLocation(LOCATION_SZONE) or not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.Equip(tp,c,tc) then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && duel::equip(d, tp, c, tc, true, false) {
        // Equip limit
        // local e0=Effect.CreateEffect(c)
        // e0:SetType(EFFECT_TYPE_SINGLE)
        // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        // e0:SetCode(EFFECT_EQUIP_LIMIT)
        // e0:SetValue(1)
        // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
        // c:RegisterEffect(e0)
        let e0 = effect::create_effect(d, c);
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
        effect::set_value(d, e0, 1);
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e0, false);
        // ● The equipped monster is unaffected by Spell effects
        // local e1=Effect.CreateEffect(c)
        // e1:SetType(EFFECT_TYPE_EQUIP)
        // e1:SetCode(EFFECT_IMMUNE_EFFECT)
        // e1:SetValue(function(e,te) return te:IsSpellEffect() end)
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        // c:RegisterEffect(e1)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e1, lua_fn(immune_value));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        // ● Once per turn: You can destroy this card, and if you do, add 1 "Malefic" card from your Deck to your hand
        // local e2=Effect.CreateEffect(c)
        // e2:SetDescription(aux.Stringid(id,2))
        // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
        // e2:SetType(EFFECT_TYPE_QUICK_O)
        // e2:SetCode(EVENT_FREE_CHAIN)
        // e2:SetRange(LOCATION_SZONE)
        // e2:SetCountLimit(1)
        // e2:SetTarget(s.desthtg)
        // e2:SetOperation(s.desthop)
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
        // c:RegisterEffect(e2)
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, aux::stringid(ID, 2));
        effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
        effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
        effect::set_code(d, e2, EVENT_FREE_CHAIN);
        effect::set_range(d, e2, LOCATION_SZONE);
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        effect::set_target(d, e2, Some(cost_target(desthtg)));
        effect::set_operation(d, e2, Some(operation(desthop)));
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_hint_timing(d, e2, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
        card::register_effect(d, c, e2, false);
    }
}
// function(e,te) return te:IsSpellEffect() end
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::is_spell_effect(d, te)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.desthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn desthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.desthop(e,tp,eg,ep,ev,re,r,rp)
fn desthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
// e2:SetCondition(function() return Duel.IsExistingMatchingCard(nil,0,LOCATION_FZONE,LOCATION_FZONE,1,nil) end)
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, None, 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
// function s.descostfilter(c)
fn descostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.descost(e,tp,eg,ep,ev,re,r,rp,chk)
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.descostfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&descostfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.descostfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_matching_card(d, tp, Some(&descostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g+c,POS_FACEUP,REASON_COST)
    let mut gc = g;
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::remove(d, gc, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
