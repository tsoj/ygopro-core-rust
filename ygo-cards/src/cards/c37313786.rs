//! Gamble
use super::prelude::*;

pub const ID: u32 = 37313786;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_COIN+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)<=2 and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>=6
    duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) <= 2 && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) >= 6
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.CallCoin(tp) then
    if duel::call_coin(d, a.tp) {
        // local gc=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
        let gc = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        // Duel.Draw(tp,5-gc,REASON_EFFECT)
        duel::draw(d, a.tp, (5 - gc) as u32, REASON_EFFECT);
    } else {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetCode(EFFECT_SKIP_TURN)
        effect::set_code(d, e1, EFFECT_SKIP_TURN);
        // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
