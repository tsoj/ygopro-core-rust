//! Greed Grado
use super::prelude::*;

pub const ID: u32 = 3972721;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    // local p1=false
    let mut p1 = false;
    // local p2=false
    let mut p2 = false;
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsType(TYPE_SYNCHRO) and tc:IsPreviousLocation(LOCATION_MZONE)
        //     and ((tc:IsReason(REASON_BATTLE) and (tc:GetBattlePosition()&POS_FACEUP)~=0)
        //     or (not tc:IsReason(REASON_BATTLE) and tc:IsPreviousPosition(POS_FACEUP)))
        //     and tc:GetPreviousControler()~=tc:GetReasonPlayer() then
        let battle_faceup = card::is_reason(d, tc, REASON_BATTLE) && (card::get_battle_position(d, tc) & POS_FACEUP as u32) != 0;
        let prev_faceup = !card::is_reason(d, tc, REASON_BATTLE) && card::is_previous_position(d, tc, POS_FACEUP as u32);
        if card::is_type(d, tc, TYPE_SYNCHRO)
            && card::is_previous_location(d, tc, LOCATION_MZONE)
            && (battle_faceup || prev_faceup)
            && card::get_previous_controler(d, tc) != card::get_reason_player(d, tc)
        {
            // if tc:GetReasonPlayer()==0 then p1=true else p2=true end
            if card::get_reason_player(d, tc) == 0 {
                p1 = true;
            } else {
                p2 = true;
            }
        }
    }
    // if p1 then Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1) end
    if p1 {
        duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    // if p2 then Duel.RegisterFlagEffect(1,id,RESET_PHASE|PHASE_END,0,1) end
    if p2 {
        duel::register_flag_effect(d, 1, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id)~=0
    duel::get_flag_effect(d, a.tp, ID) != 0
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
