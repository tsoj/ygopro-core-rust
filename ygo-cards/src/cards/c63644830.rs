//! Melffy Hide-and-Seek
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 63644830;

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
    // Cannot be destroyed by card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_BEAST))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_BEAST))));
    // e2:SetValue(s.indct)
    effect::set_value_fn(d, e2, lua_fn(indct));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Shuffle and draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_DRAW);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.indct(e,re,r,rp)`
fn indct(_d: &mut Duel, a: Args) -> i64 {
    // if r&REASON_EFFECT==REASON_EFFECT then
    if (a.int(2) as u32 & REASON_EFFECT) == REASON_EFFECT {
        // return 1
        1
    } else {
        // return 0
        0
    }
}
/// `s.tdfilter(c,e)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRace(RACE_BEAST) and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_race(d, c, RACE_BEAST) && card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.tdcheck(sg,e,tp)`
fn tdcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==#sg
    (group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len(), false)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc, e);
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and aux.SelectUnselectGroup(g,e,tp,3,3,s.tdcheck,0) end
    if a.chk == Some(0) {
        let rs: aux::Rescon = &tdcheck;
        return duel::is_player_can_draw(d, tp, 1)
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,s.tdcheck,1,tp,HINTMSG_TODECK)
    let rs: aux::Rescon = &tdcheck;
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), n, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // if #tg<=0 then return end
    if tg.is_empty() {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
