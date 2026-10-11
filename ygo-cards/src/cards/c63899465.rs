//! Rescue-ACE HQ
use super::prelude::*;

pub const ID: u32 = 63899465;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RESCUE_ACE],
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
    // --"Rescue-ACE" monsters you control gain 500 ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.atkcond)
    effect::set_condition(d, e2, Some(condition(atkcond)));
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_RESCUE_ACE))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_RESCUE_ACE as u16]))));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Additional Normal summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e4, EFFECT_EXTRA_SUMMON_COUNT);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_HAND | LOCATION_MZONE, 0);
    // e4:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_RESCUE_ACE))
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_RESCUE_ACE as u16]))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --Shuffle 4 "Rescue-ACE" cards and draw 1
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e5, CATEGORY_TODECK + CATEGORY_DRAW);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.tdtg)
    effect::set_target(d, e5, Some(cost_target(tdtg)));
    // e5:SetOperation(s.tdop)
    effect::set_operation(d, e5, Some(operation(tdop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atkcond(e)`
fn atkcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_MZONE)>0
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) > 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RESCUE_ACE) and c:IsAbleToDeck() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_RESCUE_ACE as u16]) && card::is_able_to_deck(d, c) && card::is_faceup(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsExistingTarget(s.cfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,4,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_existing_target(d, Some(&cfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 4, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,4,4,nil)
    let g = duel::select_target(d, a.tp, Some(&cfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 4, 4, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 or Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if tg.is_empty() {
        return;
    }
    if duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if not g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK|LOCATION_EXTRA) then return end
    if !group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX) {
        return;
    }
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
