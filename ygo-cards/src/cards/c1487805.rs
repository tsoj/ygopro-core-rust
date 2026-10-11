//! Cosmic Slicer Zer'oll
//! 宇宙鋏ゼロオル
use super::prelude::*;

pub const ID: u32 = 1487805;
pub const INFO: ScriptInfo = ScriptInfo {
    counter_list: &[COUNTER_A],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_REPTILE),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_REPTILE, sc, st, Some(tp)))), 2, None, None, None, None);
    // Search 1 card that has an effect that places A-Counter(s)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(link_summoned_cond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Normal Summon 1 Reptile monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.sumcost)
    effect::set_cost(d, e2, Some(cost_target(sumcost)));
    // e2:SetTarget(s.sumtg)
    effect::set_target(d, e2, Some(cost_target(sumtg)));
    // e2:SetOperation(s.sumop)
    effect::set_operation(d, e2, Some(operation(sumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change opponent's monsters with an A-Counter to Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SET_POSITION)
    effect::set_code(d, e3, EFFECT_SET_POSITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetTarget(function(_,c) return c:IsFaceup() and c:GetCounter(COUNTER_A)>0 end)
    effect::set_target(d, e3, Some(effect_card_fn(pos_target)));
    // e3:SetValue(POS_FACEUP_DEFENSE)
    effect::set_value(d, e3, POS_FACEUP_DEFENSE as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Opponent's monsters with an A-Counter cannot activate their effects
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e4, EFFECT_CANNOT_TRIGGER);
    // e4:SetValue(0)
    effect::set_value(d, e4, 0);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) return e:GetHandler():IsLinkSummoned() end`
fn link_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `function(_,c) return c:IsFaceup() and c:GetCounter(COUNTER_A)>0 end`
fn pos_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c) && card::get_counter(d, c, COUNTER_A as u16) > 0
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:PlacesCounter(COUNTER_A) and c:IsAbleToHand()
    card::places_counter(d, c, COUNTER_A) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
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
/// `s.sumcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,1,COUNTER_A,2,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, tp, 1, 1, COUNTER_A as u16, 2, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,1,COUNTER_A,2,REASON_COST)
    duel::remove_counter(d, tp, 1, 1, COUNTER_A as u16, 2, REASON_COST);
    true
}
/// `s.sumfilter(c)`
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_REPTILE) and c:IsSummonable(true,nil)
    card::is_race(d, c, RACE_REPTILE) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sumfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&sumfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.Summon(tp,g:GetFirst(),true,nil) end
    if !g.is_empty() {
        let first = group::get_first(d, &g).nil();
        duel::summon(d, tp, first, true, None, 0, None);
    }
}
