//! End of the Line
use super::prelude::*;

pub const ID: u32 = 53442500;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
/// `s.condition(e,tp,...)`: `return Duel.GetLP(tp)<100`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, a.tp) < 100
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetLP(tp)<10 and 4 or 2
    let ct: u32 = if duel::get_lp(d, tp) < 10 { 4 } else { 2 };
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, ct);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // if Duel.Draw(p,2,REASON_EFFECT)~=2 then return end
    if duel::draw(d, p, 2, REASON_EFFECT) != 2 {
        return;
    }
    // if Duel.GetLP(tp)<10 then
    if duel::get_lp(d, tp) < 10 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
