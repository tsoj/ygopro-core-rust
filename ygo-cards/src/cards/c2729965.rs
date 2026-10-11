//! Spatial Trunade
//! 宇宙的ハリケーン
use super::prelude::*;

pub const ID: u32 = 2729965;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return up to 2 cards on the field to the hand, then each player places cards from their hand on the bottom of the Deck in any order equal to the number returned to their hand. You can only activate 1 "Spatial Trunade" per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_EITHER,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_EITHER as u8, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, e) { c.into() } else { NO_EX };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,2,exc)
    let mut g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, exc).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.SendtoHand(g,nil,REASON_EFFECT)>0 and #g:Match(Card.IsLocation,nil,LOCATION_HAND)>0 then
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        group::match_(d, &mut g, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
        if !g.is_empty() {
            // local ct=g:FilterCount(Card.IsControler,nil,tp)
            let ct = group::filter_count(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX);
            // if ct>0 then Duel.ShuffleHand(tp) end
            if ct > 0 {
                duel::shuffle_hand(d, tp);
            }
            // if #g>ct then Duel.ShuffleHand(1-tp) end
            if g.len() > ct {
                duel::shuffle_hand(d, 1 - tp);
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // s.place_on_deck_bottom(tp,ct)
            place_on_deck_bottom(d, tp, ct);
            // s.place_on_deck_bottom(1-tp,#g-ct)
            place_on_deck_bottom(d, 1 - tp, g.len() - ct);
        }
    }
}
/// `s.place_on_deck_bottom(p,ct)`
fn place_on_deck_bottom(d: &mut Duel, p: u8, ct: usize) {
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,p,LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), p, LOCATION_HAND, 0, NO_EX);
    // if #g<ct then return end
    if g.len() < ct {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
    // local sg=g:Select(p,ct,ct,nil)
    let sg = group::select(d, &g, p, ct as u16, ct as u16, false, NO_EX).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local sort_ct=Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
    let sort_ct = duel::send_to_deck(d, sg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    // if sort_ct>1 then
    if sort_ct > 1 {
        // Duel.SortDeckbottom(p,p,sort_ct)
        duel::sort_deckbottom(d, p, p, sort_ct as u16);
    }
}
