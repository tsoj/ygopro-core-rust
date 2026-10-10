//! No.14 Greedy Sarameya
use super::prelude::*;

pub const ID: u32 = 21313376;

pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(14),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // reflect damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_REFLECT_DAMAGE)
    effect::set_code(d, e1, EFFECT_REFLECT_DAMAGE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.refcon)
    effect::set_value_fn(d, e1, lua_fn(refcon));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCondition(aux.bdogcon)
    effect::set_condition(d, e2, Some(aux::bdogcon()));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.refcon(e,re,val,r,rp,rc)`
fn refcon(d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_EFFECT)~=0 and rp==1-e:GetHandlerPlayer()
    let r = a.int(3) as u32;
    let e = a.e(0);
    (r & REASON_EFFECT) != 0 && a.p(4) == 1 - effect::get_handler_player(d, e)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local atk=e:GetHandler():GetBattleTarget():GetBaseAttack()
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).nil();
    let atk = card::get_base_attack(d, bt);
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackBelow,atk),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,atk) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_attack_below(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,atk),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_attack_below(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local atk=e:GetHandler():GetBattleTarget():GetBaseAttack()
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).nil();
    let atk = card::get_base_attack(d, bt);
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,atk),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_attack_below(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
