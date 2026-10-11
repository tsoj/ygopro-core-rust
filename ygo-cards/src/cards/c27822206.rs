//! Ages of Stars and Frost
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 27822206;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(function(e,tp) return Duel.HasFlagEffect(tp,id) and not Duel.HasFlagEffect(tp,id+1) end)
    effect::set_condition(d, e1, Some(condition(activate_cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e2, EVENT_TO_DECK);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register if the card's condition is met
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // for tc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // local prev_p=tc:GetPreviousControler()
        let prev_p = card::get_previous_controler(d, tc);
        // if tc:IsPreviousLocation(LOCATION_MZONE) and tc:IsReason(REASON_DESTROY) and tc:IsReason(REASON_EFFECT)
        //   and ((re:IsSpellEffect() and rp==prev_p) or rp==1-prev_p) then
        if card::is_previous_location(d, tc, LOCATION_MZONE) && card::is_reason(d, tc, REASON_DESTROY) && card::is_reason(d, tc, REASON_EFFECT) {
            let spell = effect::is_spell_effect(d, a.re.clone().nil());
            if (spell && a.rp == prev_p) || a.rp == 1 - prev_p {
                // Duel.RegisterFlagEffect(prev_p,id,RESET_PHASE|PHASE_END,0,1)
                duel::register_flag_effect(d, prev_p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
        }
    }
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.HasFlagEffect(tp,id+1) then return end
    if duel::has_flag_effect(d, tp, ID + 1, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local turn_ct=Duel.GetTurnCount()
    let turn_ct = duel::get_turn_count(d, None);
    // local ct=Duel.IsTurnPlayer(tp) and Duel.IsBattlePhase() and 2 or 1
    let ct: i32 = if duel::is_turn_player(d, tp) && duel::is_battle_phase(d, None) { 2 } else { 1 };
    // Can conduct your next Battle Phase twice
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_BP_TWICE)
    effect::set_code(d, e1, EFFECT_BP_TWICE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetCondition(function() return ct==1 or Duel.GetTurnCount()~=turn_ct end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(move |d, _a| {
            ct == 1 || duel::get_turn_count(d, None) != turn_ct
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_SELF_TURN,ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_SELF_TURN, ct as u16);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function(e,tp) return Duel.HasFlagEffect(tp,id) and not Duel.HasFlagEffect(tp,id+1) end
fn activate_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::has_flag_effect(d, a.tp, ID, None) && !duel::has_flag_effect(d, a.tp, ID + 1, None)
}
// s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and (c:IsLocation(LOCATION_DECK) or c:IsFacedown())
    card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && (card::is_location(d, c, LOCATION_DECK) || card::is_facedown(d, c))
}
// s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
