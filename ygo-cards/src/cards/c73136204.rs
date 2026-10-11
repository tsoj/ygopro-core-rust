//! Sylvan Lotuswain
use super::prelude::*;

pub const ID: u32 = 73136204;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SYLVAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavte cards from your Deck up to the number of cards your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return up to 5 "Sylvan" cards to the bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ac=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
        let ac = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
        // return ac>0 and Duel.IsPlayerCanDiscardDeck(tp,ac)
        return ac > 0 && duel::is_player_can_discard_deck(d, a.tp, ac as u32);
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local ac=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let mut ac = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) as i64;
    // if ac==0 or not Duel.IsPlayerCanDiscardDeck(tp,ac) then return end
    if ac == 0 || !duel::is_player_can_discard_deck(d, a.tp, ac as u32) {
        return;
    }
    // Duel.ConfirmDecktop(tp,ac)
    duel::confirm_decktop(d, a.tp, ac as u32);
    // local g=Duel.GetDecktopGroup(tp,ac)
    let g = duel::get_decktop_group(d, a.tp, ac as u32);
    // local sg=g:Filter(Card.IsRace,nil,RACE_PLANT)
    let sg = group::filter(d, &g, &|d, c| card::is_race(d, c, RACE_PLANT), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, sg.clone(), REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
    // ac=ac-#sg
    ac -= sg.len() as i64;
    // if ac>0 then
    if ac > 0 {
        // Duel.MoveToDeckBottom(ac,tp)
        duel::move_decktop_to_bottom(d, ac as u32, a.tp);
        // Duel.SortDeckbottom(tp,tp,ac)
        duel::sort_deckbottom(d, a.tp, a.tp, ac as u16);
    }
}
// function s.tdcon(e,tp,eg,ep,ev,re,r,rp)
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_DECK) and c:IsReason(REASON_EXCAVATE)
    card::is_previous_location(d, c, LOCATION_DECK) && card::is_reason(d, c, REASON_EXCAVATE)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SYLVAN) and not c:IsCode(id) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_SYLVAN as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,5,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 5, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
        let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        // if ct==0 then return end
        if ct == 0 {
            return;
        }
        // Duel.SortDecktop(tp,tp,ct)
        duel::sort_decktop(d, a.tp, a.tp, ct as u16);
        // for i=1,ct do
        for _ in 0..ct {
            // local mg=Duel.GetDecktopGroup(tp,1)
            let mg = duel::get_decktop_group(d, a.tp, 1);
            // Duel.MoveSequence(mg:GetFirst(),1)
            let first = group::get_first(d, &mg).nil();
            duel::move_sequence(d, first, 1, None);
        }
    }
}
