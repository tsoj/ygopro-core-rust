//! Simorgh Repulsion
use super::prelude::*;

pub const ID: u32 = 64002884;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Discard 1 Winged Beast monster; return all cards in your opponent's Spell & Trap Zones to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(Cost.Discard(function(c) return c:IsRace(RACE_WINGEDBEAST) end))
    effect::set_cost(d, e1, Some(cost::discard(Some(std::sync::Arc::new(|d, c, _e, _tp| card::is_race(d, c, RACE_WINGEDBEAST))), false, None, None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; reveal 1 Winged Beast monster in your hand, and if you do, reduce the Levels of monsters in your hand with that name by 1 for the rest of this turn (even after they are Summoned)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_STZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_STZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_STZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_STZONE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_STZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_STZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    }
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:IsLevelAbove(2) and not c:IsPublic()
    card::is_race(d, c, RACE_WINGEDBEAST) && card::is_level_above(d, c, 2) && !card::is_public(d, c)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.lvfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&lvfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.lvfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&lvfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, sc);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsCode,tp,LOCATION_HAND,0,nil,sc:GetCode())
    let codes = card::get_code(d, sc);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_code(d, x, &codes)), tp, LOCATION_HAND, 0, NO_EX);
    // for hc in g:Iter() do
    for hc in group::iter(d, &g) {
        // --Reduce the Levels of monsters in your hand with that name by 1 for the rest of this turn (even after they are Summoned)
        // hc:UpdateLevel(-1,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD),c)
        card::update_level(d, hc, -1, Some(RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD)), Some(c));
    }
}
