//! Raidraptor - Roost
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 64049553;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RAIDRAPTOR], listed_names: &[ID], ..ScriptInfo::DEFAULT };

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
    // Search 1 "Raidraptor" Spell/Trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 3 of your "Raidraptor" monsters on the bottom of the Deck then draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RAIDRAPTOR) and c:IsSummonLocation(LOCATION_EXTRA) and c:IsFaceup() and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16]) && card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RAIDRAPTOR) and c:IsSpellTrap() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16]) && card::is_spell_trap(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
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
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RAIDRAPTOR) and c:IsMonster() and c:IsFaceup() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16]) && card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 3, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,3,3,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 3, 3, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 3, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 or Duel.SendtoDeck(tg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)==0 then return end
    if tg.is_empty() || duel::send_to_deck(d, &tg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local dg=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let og = duel::get_operated_group(d);
    let dg = group::filter(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if #dg==0 then return end
    if dg.is_empty() {
        return;
    }
    // local ct=dg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let ct = group::filter_count(d, &dg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if ct>1 then
    if ct > 1 {
        // Duel.SortDeckbottom(tp,tp,ct)
        duel::sort_deckbottom(d, tp, tp, ct as u16);
    }
    // if Duel.IsPlayerCanDraw(tp) then
    if duel::is_player_can_draw(d, tp, 0) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
