//! Million-Century Ice Prison
use super::prelude::*;

pub const ID: u32 = 23746827;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Skip phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.skcon)
    effect::set_condition(d, e1, Some(condition(skcon)));
    // e1:SetTarget(s.sktg)
    effect::set_target(d, e1, Some(cost_target(sktg)));
    // e1:SetOperation(s.skop)
    effect::set_operation(d, e1, Some(operation(skop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.skconfilter(c,tp)`
fn skconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if c:IsReason(REASON_DESTROY) or not c:IsReason(REASON_EFFECT) then return false end
    if card::is_reason(d, c, REASON_DESTROY) || !card::is_reason(d, c, REASON_EFFECT) {
        return false;
    }
    // local re=c:GetReasonEffect()
    let re = card::get_reason_effect(d, c).nil();
    // return re:IsMonsterEffect() and re:GetHandlerPlayer()==1-tp
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    effect::is_monster_effect(d, re)
        && effect::get_handler_player(d, re) == 1 - tp
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
/// `s.skcon(e,tp,eg,ep,ev,re,r,rp)`
fn skcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and eg:IsExists(s.skconfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    duel::is_main_phase(d, None) && group::is_exists(d, &eg, &|d, c| skconfilter(d, c, tp), 1, NO_EX)
}
/// `s.sktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    let op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // Duel.SetTargetParam(op)
    duel::set_target_param(d, op as u32);
    true
}
/// `s.skop(e,tp,eg,ep,ev,re,r,rp)`
fn skop(d: &mut Duel, a: &EffArgs) {
    // local op=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let op = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if op==0 and Duel.IsMainPhase() then
    if op == 0 && duel::is_main_phase(d, None) {
        // Skip the current phase
        // Duel.SkipPhase(Duel.GetTurnPlayer(),Duel.GetCurrentPhase(),RESET_PHASE|PHASE_END,1)
        let turn_player = duel::get_turn_player(d);
        let phase = duel::get_current_phase(d);
        duel::skip_phase(d, turn_player, phase, RESET_PHASE | PHASE_END, 1, 0);
    } else if op == 1 {
        // Skip the Main Phase 1 of the opponent's next turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET|EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_SKIP_M1)
        effect::set_code(d, e1, EFFECT_SKIP_M1);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // if Duel.IsTurnPlayer(1-tp) and Duel.IsPhase(PHASE_MAIN1) then
        if duel::is_turn_player(d, 1 - a.tp) && duel::is_phase(d, PHASE_MAIN1) {
            // local turn=Duel.GetTurnCount()
            let turn = duel::get_turn_count(d, None) as i64;
            // e1:SetCondition(function() return Duel.GetTurnCount()~=turn end)
            effect::set_condition(
                d,
                e1,
                Some(lua_fn(move |d: &mut Duel, _a: Args| duel::get_turn_count(d, None) as i64 != turn)),
            );
            // e1:SetReset(RESET_PHASE|PHASE_MAIN1|RESET_OPPO_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_MAIN1 | RESET_OPPO_TURN, 2);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_MAIN1|RESET_OPPO_TURN,1)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_MAIN1 | RESET_OPPO_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
