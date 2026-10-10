//! CXyz Skypalace Babylon
use super::prelude::*;

pub const ID: u32 = 49202331;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={3814632}
    listed_names: &[3814632],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,11,3)
    xyz::add_procedure(d, c, None, Some(11), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(aux.bdgcon)
    effect::set_condition(d, e1, Some(aux::bdgcon()));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // chain attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atcon)
    effect::set_condition(d, e2, Some(condition(atcon)));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `local dam=bc:GetAttack()/2; if dam<0 then dam=0 end`: the damage as Lua computes it (a float;
/// the core rounds it half away from zero when it is used as an integer)
fn half_attack(atk: i32) -> i64 {
    let dam = atk as f64 / 2.0;
    lua_int(if dam < 0.0 { 0.0 } else { dam })
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc.nil());
    // local dam=bc:GetAttack()/2
    // if dam<0 then dam=0 end
    let dam = half_attack(card::get_attack(d, bc.nil()));
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
/// `s.damop(e,tp,...)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // local dam=tc:GetAttack()/2
        // if dam<0 then dam=0 end
        let dam = half_attack(card::get_attack(d, tc));
        // Duel.Damage(p,dam,REASON_EFFECT)
        duel::damage(d, p, dam, REASON_EFFECT, false, None);
    }
}
/// `s.atcon(e,tp,eg,ep,ev,re,r,rp)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return aux.bdcon(e,tp,eg,ep,ev,re,r,rp) and c:CanChainAttack()
    //     and c:GetOverlayGroup():IsExists(Card.IsCode,1,nil,3814632)
    aux::bdcon().call(d, &[Value::Effect(a.e)]).truthy()
        && card::can_chain_attack(d, c, None, false)
        && group::is_exists(d, &card::get_overlay_group(d, c), &|d, x| card::is_code(d, x, &[3814632]), 1, NO_EX)
}
/// `s.atop(e,tp,...)`
fn atop(d: &mut Duel, _a: &EffArgs) {
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
