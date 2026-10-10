//! Number C9: Chaos Dyson Sphere
use super::prelude::*;

pub const ID: u32 = 32559361;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[1992816], xyz_number: Some(9), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 10 monsters
    // Xyz.AddProcedure(c,nil,10,3)
    xyz::add_procedure(d, c, None, Some(10), 3, None, None, None, None, false, None);
    // Attach an opponent's monster this card battles to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Inflict 300 damage to your opponent for each material attached to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.dam300tg)
    effect::set_target(d, e2, Some(cost_target(dam300tg)));
    // e2:SetOperation(s.dam300op)
    effect::set_operation(d, e2, Some(operation(dam300op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict 800 damage to your opponent for each material detached
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,1992816) end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetCost(Cost.DetachFromSelf(1,function(e) return e:GetHandler():GetOverlayCount() end,function(e,og) e:SetLabel(#og) end))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::F(std::sync::Arc::new(overlay_count_fn))), Some(std::sync::Arc::new(set_label_count_fn)))));
    // e3:SetTarget(s.dam800tg)
    effect::set_target(d, e3, Some(cost_target(dam800tg)));
    // e3:SetOperation(s.dam800op)
    effect::set_operation(d, e3, Some(operation(dam800op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():GetOverlayCount() end` (the max of Cost.DetachFromSelf)
fn overlay_count_fn(d: &mut Duel, e: EffectId, _tp: u8) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_overlay_count(d, h) as i64
}
/// `function(e,og) e:SetLabel(#og) end` (the operation of Cost.DetachFromSelf)
fn set_label_count_fn(d: &mut Duel, e: EffectId, og: &CardSet) {
    effect::set_label(d, e, &[og.len() as i64]);
}
/// `function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,1992816) end`
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d, c| card::is_code(d, c, &[1992816]), 1, NO_EX)
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local bc=c:GetBattleTarget()
        let bc = card::get_battle_target(d, c);
        // return bc and bc:IsControler(1-tp) and bc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
        return match bc {
            Some(bc) => card::is_controler(d, bc, 1 - a.tp) && card::is_can_be_xyz_material(d, bc, Some(c), Some(a.tp), Some(REASON_EFFECT)),
            None => false,
        };
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if c:IsRelateToEffect(e) and bc:IsRelateToBattle() and not bc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let bc = bc.nil();
        if card::is_relate_to_battle(d, bc) && !card::is_immune_to_effect(d, bc, a.e) {
            // Duel.Overlay(c,bc,true)
            duel::overlay(d, c, bc, true);
        }
    }
}
/// `s.dam300tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dam300tg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetHandler():GetOverlayCount()
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_overlay_count(d, h) as i32;
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, ct * 300);
    true
}
/// `s.dam300op(e,tp,eg,ep,ev,re,r,rp)`
fn dam300op(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=e:GetHandler():GetOverlayCount()
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_overlay_count(d, h) as i64;
    // Duel.Damage(p,ct*300,REASON_EFFECT)
    duel::damage(d, p, ct * 300, REASON_EFFECT, false, None);
}
/// `s.dam800tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dam800tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (ct * 800) as i32);
    true
}
/// `s.dam800op(e,tp,eg,ep,ev,re,r,rp)`
fn dam800op(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.Damage(p,ct*800,REASON_EFFECT)
    duel::damage(d, p, ct * 800, REASON_EFFECT, false, None);
}
