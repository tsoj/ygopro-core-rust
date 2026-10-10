//! Sphere Kuriboh
use super::prelude::*;

pub const ID: u32 = 33245030;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pos
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ritual material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_EXTRA_RITUAL_MATERIAL)
    effect::set_code(d, e2, EFFECT_EXTRA_RITUAL_MATERIAL);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(lua_fn(con)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: Args) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_player_affected_by_effect(d, p, CARD_SPIRIT_ELIMINATION).is_none()
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp)
    let att = duel::get_attacker(d).nil();
    card::is_controler(d, att, 1 - a.tp)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    //     local at=Duel.GetAttacker()
    //     return at:IsAttackPos() and at:IsCanChangePosition()
    // end
    if a.chk == Some(0) {
        let at = duel::get_attacker(d).nil();
        return card::is_attack_pos(d, at) && card::is_can_change_position(d, at);
    }
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if at and at:IsAttackPos() and at:IsRelateToBattle() then
    if let Some(at) = at {
        if card::is_attack_pos(d, at) && card::is_relate_to_battle(d, at) {
            // Duel.ChangePosition(at,POS_FACEUP_DEFENSE)
            duel::change_position(d, at, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
}
