//! Sennet Prophet - Amenhotep
use super::prelude::*;

pub const ID: u32 = 100458002;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZENET],
    ..ScriptInfo::DEFAULT
};

/// `LOCATIONS_HAND_DECK_ONFIELD_GRAVE`
const LOCATIONS_HAND_DECK_ONFIELD_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1a, CATEGORY_SET);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1a, Some(cost::self_to_grave()));
    // e1a:SetTarget(s.settg)
    effect::set_target(d, e1a, Some(cost_target(settg)));
    // e1a:SetOperation(s.setop)
    effect::set_operation(d, e1a, Some(operation(setop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.showfilter(c)`
fn showfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsOriginalType(TYPE_NORMAL) and (c:IsFaceup() or not c:IsOnField())
    card::is_original_type(d, c, TYPE_NORMAL) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_ONFIELD))
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZENET) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_ZENET as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&showfilter), a.tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local show_group=Duel.GetMatchingGroup(s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,nil)
    let show_group = duel::get_matching_group(d, Some(&showfilter), tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, NO_EX);
    // if #show_group==0 then return end
    if show_group.is_empty() {
        return;
    }
    // local set_group=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
    let set_group = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #set_group==0 then return end
    if set_group.is_empty() {
        return;
    }
    // local szone_count=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let szone_count = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local max_count=math.min(2,szone_count,#show_group,#set_group)
    let max_count = 2.min(szone_count).min(show_group.len() as i32).min(set_group.len() as i32);
    // if max_count==0 then return end
    if max_count == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,1,max_count,nil)
    let rg = duel::select_matching_card(d, tp, Some(&showfilter), tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, 1, max_count as u16, false, NO_EX).nil();
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // local confirm_group,hintselection_group=rg:Split(Card.IsLocation,nil,LOCATION_HAND|LOCATION_DECK)
    let (confirm_group, hintselection_group) = group::split(d, &rg, &|d, c| card::is_location(d, c, LOCATION_HAND | LOCATION_DECK), NO_EX);
    // if #hintselection_group>0 then Duel.HintSelection(hintselection_group) end
    if !hintselection_group.is_empty() {
        duel::hint_selection(d, hintselection_group, true);
    }
    // if #confirm_group>0 then
    if !confirm_group.is_empty() {
        // Duel.ConfirmCards(1-tp,confirm_group)
        duel::confirm_cards(d, 1 - tp, confirm_group.clone());
        // if confirm_group:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then Duel.ShuffleHand(tp) end
        if group::is_exists(d, &confirm_group, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX) {
            duel::shuffle_hand(d, tp);
        }
        // if confirm_group:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
        if group::is_exists(d, &confirm_group, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, tp);
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,#rg,#rg,nil)
    let rc = rg.len() as u16;
    let sg = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, rc, rc, false, NO_EX).nil();
    // if #sg==#rg then
    if sg.len() == rg.len() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SSet(tp,sg)
        duel::s_set(d, tp, sg, None, true);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsAbleToDeck()
    card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,3,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 3, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
