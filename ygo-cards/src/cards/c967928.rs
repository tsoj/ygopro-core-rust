//! Penalty Game!
use super::prelude::*;

pub const ID: u32 = 967928;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetHintTiming(0,TIMING_TOHAND)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND));
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==4
    duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) == 4
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    let op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetCode(EFFECT_CANNOT_DRAW)
        effect::set_code(d, e1, EFFECT_CANNOT_DRAW);
        // if Duel.GetTurnPlayer()==p and Duel.IsPhase(PHASE_DRAW) then
        if duel::get_turn_player(d) == p && duel::is_phase(d, PHASE_DRAW) {
            // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 2);
            // e1:SetLabel(Duel.GetTurnCount())
            effect::set_label(d, e1, &[duel::get_turn_count(d, None) as i64]);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 1);
            // e1:SetLabel(0)
            effect::set_label(d, e1, &[0]);
        }
        // e1:SetCondition(s.skipcon)
        effect::set_condition(d, e1, Some(lua_fn(skipcon)));
        // Duel.RegisterEffect(e1,p)
        duel::register_effect(d, e1, p);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DRAW_COUNT)
        effect::set_code(d, e2, EFFECT_DRAW_COUNT);
        // e2:SetValue(0)
        effect::set_value(d, e2, 0);
        // Duel.RegisterEffect(e2,p)
        duel::register_effect(d, e2, p);
    } else {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,p)
        duel::register_effect(d, e1, p);
    }
}
/// `s.skipcon(e)`
fn skipcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel() and Duel.IsPhase(PHASE_DRAW)
    let e = a.e(0);
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, e) && duel::is_phase(d, PHASE_DRAW)
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE)
}
