//! The Toil of the Normal
use super::prelude::*;

pub const ID: u32 = 83404468;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[83404468],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DISABLE+CATEGORY_TODECK+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DISABLE + CATEGORY_TODECK + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_GRAVE,0,nil,TYPE_NORMAL):GetClassCount(Card.GetCode)`
fn class_count(d: &mut Duel, tp: u8) -> usize {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_NORMAL)), tp, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64)
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNegatableMonster() and c:IsType(TYPE_EFFECT)
    card::is_negatable_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EFFECT) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_type(d, c, TYPE_EFFECT) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsAbleToHand()
    card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsType,tp,LOCATION_GRAVE,0,1,nil,TYPE_NORMAL) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_NORMAL)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_GRAVE,0,nil,TYPE_NORMAL):GetClassCount(Card.GetCode)
    let ct = class_count(d, a.tp);
    // if ct>=1 then Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,nil,0,tp,800) end
    if ct >= 1 {
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 0, a.tp, 800);
    }
    // if ct>=2 then Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,tp,LOCATION_MZONE) end
    if ct >= 2 {
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, a.tp, LOCATION_MZONE as i32);
    }
    // if ct>=4 then Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_MZONE) end
    if ct >= 4 {
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_MZONE as i32);
    }
    // if ct>=5 then Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK) end
    if ct >= 5 {
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,nil,0,tp,800)
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 0, a.tp, 800);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_GRAVE,0,nil,TYPE_NORMAL):GetClassCount(Card.GetCode)
    let ct = class_count(d, a.tp);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local breakeff=false
    let mut breakeff = false;
    // if ct>=1 then
    if ct >= 1 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetTarget(function(e,c) return c:IsType(TYPE_NORMAL) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_NORMAL))));
        // e1:SetValue(800)
        effect::set_value(d, e1, 800);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,1))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
        // breakeff=true
        breakeff = true;
    }
    // if ct>=2 and Duel.IsExistingMatchingCard(s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) then
    if ct >= 2 && duel::is_existing_matching_card(d, Some(&disfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&disfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc then
        if let Some(sc) = sc {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
            // if breakeff then Duel.BreakEffect() end
            if breakeff {
                duel::break_effect(d);
            }
            // sc:NegateEffects(c,RESET_PHASE|PHASE_END)
            card::negate_effects(d, sc, c, Some(RESET_PHASE | PHASE_END), false, None);
            // Duel.AdjustInstantly(sc)
            duel::adjust_instantly(d, Some(sc));
        }
        // breakeff=true
        breakeff = true;
    }
    // if ct>=3 then
    if ct >= 3 {
        // if breakeff then Duel.BreakEffect() end
        if breakeff {
            duel::break_effect(d);
        }
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // e2:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e2, LOCATION_MZONE, 0);
        // e2:SetTarget(function(e,c) return c:IsType(TYPE_NORMAL) end)
        effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_NORMAL))));
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
        // breakeff=true
        breakeff = true;
    }
    // if ct>=4 and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) then
    if ct >= 4 && duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // if breakeff then Duel.BreakEffect() end
            if breakeff {
                duel::break_effect(d);
            }
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
        // breakeff=true
        breakeff = true;
    }
    // if ct>=5 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) then
    if ct >= 5 && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // if breakeff then Duel.BreakEffect() end
            if breakeff {
                duel::break_effect(d);
            }
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
