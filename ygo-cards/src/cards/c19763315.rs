//! An Unfortunate Report
//! 不運なリポート
use super::prelude::*;

pub const ID: u32 = 19763315;

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
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_BP_TWICE)
    effect::set_code(d, e1, EFFECT_BP_TWICE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // if Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase() then
    if duel::is_turn_player(d, 1 - tp) && duel::is_battle_phase(d, None) {
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e1, &[turn]);
        // e1:SetCondition(s.bpcon)
        effect::set_condition(d, e1, Some(lua_fn(bpcon)));
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_OPPO_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_OPPO_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_OPPO_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_OPPO_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.bpcon(e)`
fn bpcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e(0))
}
