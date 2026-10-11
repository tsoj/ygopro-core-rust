//! Deskbot Base
use super::prelude::*;

pub const ID: u32 = 12215894;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DESKBOT], ..ScriptInfo::DEFAULT };

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
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_DESKBOT))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_DESKBOT as u16]))));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // defup
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // draw
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_DRAW);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // todeck
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e5, CATEGORY_TODECK);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCost(s.cost2)
    effect::set_cost(d, e5, Some(cost_target(cost2)));
    // e5:SetTarget(s.target2)
    effect::set_target(d, e5, Some(cost_target(target2)));
    // e5:SetOperation(s.operation2)
    effect::set_operation(d, e5, Some(operation(operation2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DESKBOT) and c:IsAbleToDeck() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_DESKBOT as u16]) && card::is_able_to_deck(d, c) && !card::is_public(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 0) && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(p,s.filter,p,LOCATION_HAND,0,1,99,nil)
    let g = duel::select_matching_card(d, p, Some(&filter), p, LOCATION_HAND, 0, 1, 99, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(1-p,g)
        duel::confirm_cards(d, 1 - p, g.clone());
        // local ct=Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        let ct = duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // Duel.ShuffleDeck(p)
        duel::shuffle_deck(d, p);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(p,ct,REASON_EFFECT)
        duel::draw(d, p, ct as u32, REASON_EFFECT);
        // Duel.ShuffleHand(p)
        duel::shuffle_hand(d, p);
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DESKBOT) and c:IsAbleToRemoveAsCost() and not c:IsCode(id)
    //     and (c:IsLocation(LOCATION_SZONE) or aux.SpElimFilter(c,true,true))
    card::is_set_card(d, c, &[SET_DESKBOT as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && !card::is_code(d, c, &[ID])
        && (card::is_location(d, c, LOCATION_SZONE) || aux::sp_elim_filter(d, c, true, true))
}
/// `s.cost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE|LOCATION_ONFIELD,0,nil)
    let rg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE | LOCATION_ONFIELD, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,9,9,aux.dncheck,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(9), Some(9), Some(&aux::dncheck()), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local gp=aux.SelectUnselectGroup(rg,e,tp,9,9,aux.dncheck,1,tp,HINTMSG_REMOVE)
    let gp = aux::select_unselect_group(
        d,
        &rg,
        Some(a.e),
        a.tp,
        Some(9),
        Some(9),
        Some(&aux::dncheck()),
        1,
        a.tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.Remove(gp,POS_FACEUP,REASON_COST)
    duel::remove(d, gp, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_deck(d, c)),
            a.tp,
            0,
            LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.operation2(e,tp,eg,ep,ev,re,r,rp)`
fn operation2(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_HAND,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_HAND, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
