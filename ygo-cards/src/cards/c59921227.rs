//! Blitzclique Alternator
use super::prelude::*;

pub const ID: u32 = 59921227;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_HIDEOUT_IN_THE_SKY_COULOMB], listed_series: &[SET_BLITZCLIQUE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e0, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e0:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e0, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP));
    // e0:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e0, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Monsters your opponent controls lose 300 ATK/DEF for each Thunder monster you control
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1a, LOCATION_SZONE);
    // e1a:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1a, 0, LOCATION_MZONE);
    // e1a:SetValue(function(e,c) return -300*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_THUNDER),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil) end)
    effect::set_value_fn(d, e1a, effect_card_value(|d, e, _c| {
        let p = effect::get_handler_player(d, e);
        -300 * duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_THUNDER)), p, LOCATION_MZONE, 0, NO_EX) as i64
    }));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // You can shuffle 1 other "Blitzclique" card from your hand or face-up field into the Deck; add 1 Thunder monster with a different name from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.thunderthcost)
    effect::set_cost(d, e2, Some(cost_target(thunderthcost)));
    // e2:SetTarget(s.thunderthtg)
    effect::set_target(d, e2, Some(cost_target(thunderthtg)));
    // e2:SetOperation(s.thunderthop)
    effect::set_operation(d, e2, Some(operation(thunderthop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can banish this card from your GY; add 1 "Blitzclique" Spell or "Kowloon, Citadel of the Sky" from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.blitzthtg)
    effect::set_target(d, e3, Some(cost_target(blitzthtg)));
    // e3:SetOperation(s.blitzthop)
    effect::set_operation(d, e3, Some(operation(blitzthop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thunderthcostfilter(c,tp)
fn thunderthcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BLITZCLIQUE) and c:IsAbleToDeckAsCost() and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    //     and Duel.IsExistingMatchingCard(s.thunderthfilter,tp,LOCATION_DECK,0,1,nil,c:GetCode())
    card::is_set_card(d, c, &[SET_BLITZCLIQUE as u16])
        && card::is_able_to_deck_as_cost(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && {
            let code = card::get_code(d, c)[0];
            duel::is_existing_matching_card(d, Some(&|d, x| thunderthfilter(d, x, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
// function s.thunderthfilter(c,code)
fn thunderthfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsRace(RACE_THUNDER) and not c:IsCode(code) and c:IsAbleToHand()
    card::is_race(d, c, RACE_THUNDER) && !card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
// function s.thunderthcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thunderthcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thunderthcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| thunderthcostfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thunderthcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,c,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| thunderthcostfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    let sc = group::get_first(d, &g).nil();
    // if sc:IsLocation(LOCATION_HAND) then
    if card::is_location(d, sc, LOCATION_HAND) {
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, sc);
    // else
    } else {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
    }
    // Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, sc, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    // e:SetLabel(sc:GetCode())
    let code = card::get_code(d, sc)[0];
    effect::set_label(d, a.e, &[code as i64]);
    true
}
// function s.thunderthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thunderthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thunderthop(e,tp,eg,ep,ev,re,r,rp)
fn thunderthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thunderthfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let code = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, tp, Some(&|d, x| thunderthfilter(d, x, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.blitzthfilter(c)
fn blitzthfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_BLITZCLIQUE) and c:IsSpell()) or c:IsCode(CARD_HIDEOUT_IN_THE_SKY_COULOMB)) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_BLITZCLIQUE as u16]) && card::is_spell(d, c)) || card::is_code(d, c, &[CARD_HIDEOUT_IN_THE_SKY_COULOMB]))
        && card::is_able_to_hand(d, c, None)
}
// function s.blitzthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn blitzthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.blitzthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&blitzthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.blitzthop(e,tp,eg,ep,ev,re,r,rp)
fn blitzthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.blitzthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&blitzthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
