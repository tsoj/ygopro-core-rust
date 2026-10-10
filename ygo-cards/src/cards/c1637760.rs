//! Grand Horn of Heaven
use super::prelude::*;

pub const ID: u32 = 1637760;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1, EVENT_SPSUMMON);
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
    // return ep==1-tp and Duel.GetCurrentChain(true)==0 and Duel.IsTurnPlayer(1-tp)
    //     and Duel.IsMainPhase()
    a.ep == 1 - a.tp && duel::get_current_chain(d, true) == 0 && duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let eg = a.eg.clone().nil();
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
    // Duel.SkipPhase(1-tp,Duel.GetCurrentPhase(),RESET_PHASE|PHASE_END,1)
    let ph = duel::get_current_phase(d);
    duel::skip_phase(d, 1 - a.tp, ph, RESET_PHASE | PHASE_END, 1, 0);
}
