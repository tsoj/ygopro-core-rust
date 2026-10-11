//! Dr. Frankenderp
use super::prelude::*;

pub const ID: u32 = 27995943;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Look at the top card from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0;
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, &g);
    // local b1=g:GetFirst():IsAbleToHand()
    let first = group::get_first(d, &g).nil();
    let b1 = card::is_able_to_hand(d, first, None);
    // local op=Duel.SelectEffect(tp,
    //     {true,aux.Stringid(id,1)},
    //     {b1,aux.Stringid(id,2)})
    let sel = duel::select_effect(d, tp, &[(true, aux::stringid(ID, 1)), (b1, aux::stringid(ID, 2))]);
    // if op==1 then
    if sel == Some(1) {
        // Duel.MoveSequence(g:GetFirst(),1)
        let first = group::get_first(d, &g).nil();
        duel::move_sequence(d, first, 1, None);
    } else {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_SKIP_DP)
        effect::set_code(d, e1, EFFECT_SKIP_DP);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_DRAW) then
        if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_DRAW) {
            // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 2);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
