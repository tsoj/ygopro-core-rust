//! Scrubbed Raid
use super::prelude::*;

pub const ID: u32 = 79205581;

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
    // End the Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCondition(s.bpcon)
    effect::set_condition(d, e2, Some(condition(bpcon)));
    // e2:SetCost(s.bpcost)
    effect::set_cost(d, e2, Some(cost_target(bpcost)));
    // e2:SetOperation(s.bpop)
    effect::set_operation(d, e2, Some(operation(bpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.bpcon(e,tp,eg,ep,ev,re,r,rp)`
fn bpcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and Duel.GetCurrentPhase()<PHASE_BATTLE
    duel::is_battle_phase(d, None) && duel::get_current_phase(d) < PHASE_BATTLE
}
/// `s.bpcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bpcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    //     and e:GetHandler():GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_ONFIELD, 0, 1, h.into())
            && card::get_flag_effect(d, h, ID) == 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(d, tp, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_ONFIELD, 0, 1, 1, false, h.into()).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESET_CHAIN,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.bpop(e,tp,eg,ep,ev,re,r,rp)`
fn bpop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
    let tp = duel::get_turn_player(d);
    duel::skip_phase(d, tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
}
