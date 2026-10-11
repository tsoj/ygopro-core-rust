//! Xyz Armor Torpedo
use super::prelude::*;

pub const ID: u32 = 94151981;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // Cannot attack unless it has material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayCount()==0 end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e2:SetTarget(s.drwtg)
    effect::set_target(d, e2, Some(cost_target(drwtg)));
    // e2:SetOperation(s.drwop)
    effect::set_operation(d, e2, Some(operation(drwop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your opponent cannot activate cards or effects if the equipped monster battles
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.btlcond)
    effect::set_condition(d, e3, Some(condition(btlcond)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate the effects of your opponent's monsters if the equipped monster battles
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e4, EFFECT_DISABLE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetCondition(s.btlcond)
    effect::set_condition(d, e4, Some(condition(btlcond)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Your opponent cannot target the equipped monster if it is an Xyz monster
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
    // e5:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e5, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e5:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e5:SetCondition(function(e) return e:GetHandler():GetEquipTarget():IsType(TYPE_XYZ) end)
    effect::set_condition(d, e5, Some(condition(cond5)));
    // e5:SetValue(aux.tgoval)
    effect::set_value_fn(d, e5, aux::tgoval());
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Adjust itself to apply the negation effect right away
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e6:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e6:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e6, EVENT_BE_BATTLE_TARGET);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e6, Some(condition(cond6)));
    // e6:SetOperation(function(e) Duel.AdjustInstantly(e:GetHandler()) end)
    effect::set_operation(d, e6, Some(operation(op6)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `function(e) return e:GetHandler():GetOverlayCount()==0 end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) == 0
}
/// `s.drwtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drwtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drwop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drwop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.btlcond(e)`
fn btlcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // return ec and (Duel.GetAttacker()==ec or Duel.GetAttackTarget()==ec)
    match ec {
        None => false,
        Some(ec) => duel::get_attacker(d) == Some(ec) || duel::get_attack_target(d) == Some(ec),
    }
}
/// `function(e) return e:GetHandler():GetEquipTarget():IsType(TYPE_XYZ) end`
fn cond5(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let t = card::get_equip_target(d, h).nil();
    card::is_type(d, t, TYPE_XYZ)
}
/// `function(e) return e:GetHandler():GetEquipTarget() end`
fn cond6(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some()
}
/// `function(e) Duel.AdjustInstantly(e:GetHandler()) end`
fn op6(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::adjust_instantly(d, Some(h));
}
