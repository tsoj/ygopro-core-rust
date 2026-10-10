//! Dragunity Knight - Gae Dearg
use super::prelude::*;

pub const ID: u32 = 34116027;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_WINGEDBEAST),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_DRAGON, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_WINGEDBEAST, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_DRAGON|RACE_WINGEDBEAST) and c:IsAbleToHand()
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_DRAGON | RACE_WINGEDBEAST) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, a.tp, 1);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.SendtoHand(g,nil,REASON_EFFECT)
    duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.DiscardHand(tp,Card.IsRace,1,1,REASON_EFFECT|REASON_DISCARD,nil,RACE_DRAGON|RACE_WINGEDBEAST)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_race(d, c, RACE_DRAGON | RACE_WINGEDBEAST)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
