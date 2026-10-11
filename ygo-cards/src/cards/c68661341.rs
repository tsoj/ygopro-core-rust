//! Single Purchase
use super::prelude::*;

pub const ID: u32 = 68661341;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() or not c:IsAbleToRemoveAsCost()
    card::is_monster(d, c) || !card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let mut g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // g:RemoveCard(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    group::remove_card(&mut g, &CardSet::from_card(h));
    // if chk==0 then return #g>=3 and not g:IsExists(s.cfilter,1,nil) end
    if a.chk == Some(0) {
        return g.len() >= 3 && !group::is_exists(d, &g, &|d, c| cfilter(d, c), 1, NO_EX);
    }
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(s.sumlimit)
        effect::set_target(d, e1, Some(effect_card_fn(sumlimit)));
        // e1:SetLabel(g:GetFirst():GetCode())
        let first = group::get_first(d, &g).nil();
        let codes: Vec<i64> = card::get_code(d, first).iter().map(|&x| x as i64).collect();
        effect::set_label(d, e1, &codes);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
        aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    }
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetCode()~=e:GetLabel()
    let label = effect::get_label(d, e);
    card::get_code(d, c).first().map_or(true, |&code| code as i64 != label)
}
