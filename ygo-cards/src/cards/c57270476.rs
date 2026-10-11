//! Grave Lure
use super::prelude::*;

pub const ID: u32 = 57270476;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Turn the top card of your opponent's Deck face-up, then your opponent shuffles their Deck.
    //   When your opponent draws the face-up card, immediately send it to the Graveyard
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local opp=1-tp
    let opp = 1 - a.tp;
    // local top_c=Duel.GetDecktopGroup(opp,1):GetFirst()
    let top_group = duel::get_decktop_group(d, opp, 1);
    let top_c = group::get_first(d, &top_group);
    // if not top_c then return end
    let Some(top_c) = top_c else {
        return;
    };
    // Duel.ConfirmDecktop(opp,1)
    let _ = duel::confirm_decktop(d, opp, 1);
    // top_c:ReverseInDeck()
    card::reverse_in_deck(d, top_c);
    // Duel.ShuffleDeck(opp)
    duel::shuffle_deck(d, opp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetOperation(function() ... end)
    effect::set_operation(
        d,
        e1,
        Some(operation(move |d: &mut Duel, _a: &EffArgs| {
            // if not Duel.IsChainSolving() then
            if !duel::is_chain_solving(d) {
                // Duel.Hint(HINT_CARD,0,id)
                duel::hint(d, HINT_CARD as u8, 0, ID as u64);
                // Duel.SendtoGrave(top_c,REASON_EFFECT,PLAYER_NONE,tp)
                duel::send_to_grave(d, top_c, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(tp));
            } else {
                // --If it's drawn while resolving a Chain, send it to the GY at the end of the Chain Link
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e2:SetCode(EVENT_CHAIN_SOLVED)
                effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
                // e2:SetRange(LOCATION_HAND)
                effect::set_range(d, e2, LOCATION_HAND);
                // e2:SetOperation(function() Duel.Hint(HINT_CARD,0,id) Duel.SendtoGrave(top_c,REASON_EFFECT,PLAYER_NONE,tp) e2:Reset() end)
                effect::set_operation(
                    d,
                    e2,
                    Some(operation(move |d: &mut Duel, a: &EffArgs| {
                        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
                        duel::send_to_grave(d, top_c, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(tp));
                        // e2:Reset()
                        effect::reset(d, a.e);
                    })),
                );
                // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOHAND)|RESET_CHAIN)
                effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOHAND) | RESET_CHAIN, 1);
                // top_c:RegisterEffect(e2)
                card::register_effect(d, top_c, e2, false);
            }
        })),
    );
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOHAND))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOHAND), 1);
    // top_c:RegisterEffect(e1)
    card::register_effect(d, top_c, e1, false);
}
