//! Way Where There's a Will
//! 仇すれば通図
use super::prelude::*;

pub const ID: u32 = 91880660;

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
    // Excavate cards from the top of the Deck, add 1 to the hand and return the others and 1 from the hand to the bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_BOTH_SIDE)
    effect::set_property(d, e2, EFFECT_FLAG_BOTH_SIDE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.excttg)
    effect::set_target(d, e2, Some(cost_target(excttg)));
    // e2:SetOperation(s.exctop)
    effect::set_operation(d, e2, Some(operation(exctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Shuffle all cards on their opponent's field and GY into the Deck.
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_BOTH_SIDE)
    effect::set_property(d, e3, EFFECT_FLAG_BOTH_SIDE, 0);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return e:GetHandler():HasFlagEffect(id) and Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.excttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn excttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
        let ct = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
        // if ct==0 then return false end
        if ct == 0 {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,ct)
        let g = duel::get_decktop_group(d, tp, ct as u32);
        // return g:FilterCount(Card.IsAbleToHand,nil)>0
        return group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.exctop(e,tp,eg,ep,ev,re,r,rp)`
fn exctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=math.min(Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD),Duel.GetFieldGroupCount(tp,LOCATION_DECK,0))
    let ct1 = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    let ct2 = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    let mut ct = ct1.min(ct2);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // ct=ct==1 and 1 or Duel.AnnounceNumberRange(tp,1,ct)
    ct = if ct == 1 { 1 } else { duel::announce_number_range(d, tp, Some(1), Some(ct as u32), &[]).nil().0 as i32 };
    // Duel.ConfirmDecktop(tp,ct)
    duel::confirm_decktop(d, tp, ct as u32);
    // Register a flag if 9+ cards were excavated
    // if ct>=9 then
    if ct >= 9 {
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    // local thg=Duel.GetDecktopGroup(tp,ct)
    let thg = duel::get_decktop_group(d, tp, ct as u32);
    // local decrease=0
    let mut decrease = 0;
    // if #thg>0 then
    if !thg.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sc=thg:GetFirst()
        let mut sc = group::get_first(d, &thg).nil();
        // if #thg>1 then sc=thg:Select(tp,1,1,nil):GetFirst() end
        if thg.len() > 1 {
            let sel = group::select(d, &thg, tp, 1, 1, false, NO_EX).nil();
            sc = group::get_first(d, &sel).nil();
        }
        // if sc:IsAbleToHand() then
        if card::is_able_to_hand(d, sc, None) {
            // Duel.SendtoHand(sc,nil,REASON_EFFECT)
            duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
        } else {
            // Duel.SendtoGrave(sc,REASON_RULE)
            duel::send_to_grave(d, sc, REASON_RULE, None, None);
        }
        // decrease=1
        decrease = 1;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Place 1 card from the hand at the top of the deck first
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
    // Move the cards to the bottom of the Deck
    // local sortct=ct-decrease+1
    let sortct = ct - decrease + 1;
    // Duel.MoveToDeckBottom(sortct,tp)
    duel::move_decktop_to_bottom(d, sortct as u32, tp);
    // Duel.SortDeckbottom(tp,tp,sortct)
    duel::sort_deckbottom(d, tp, tp, sortct as u16);
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if #g>0 then Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `function(e,tp) return e:GetHandler():HasFlagEffect(id) and Duel.IsTurnPlayer(tp) end` (e3's condition)
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None) && duel::is_turn_player(d, a.tp)
}
