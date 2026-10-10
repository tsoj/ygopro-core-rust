//! Number C73: Abyss Supra Splash
use super::prelude::*;

pub const ID: u32 = 96864105;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(73), listed_names: &[36076683], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 6 monsters
    // Xyz.AddProcedure(c,nil,6,3)
    xyz::add_procedure(d, c, None, Some(6), 3, None, None, None, None, false, None);
    // Your battling monster gains ATK equal to the ATK of the opponent's monster it is battling, during that damage calculation only
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(1),Cost.SoftOncePerBattle(id)))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost::soft_once_per_battle(ID, None)])));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card cannot be destroyed by card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,36076683) end)
    effect::set_condition(d, e2, Some(condition(indes_cond)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn indes_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,36076683)
    let c = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[36076683]), 1, NO_EX)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    // return bc1 and bc2 and bc2:GetAttack()>0
    let (bc1, bc2) = duel::get_battle_monster(d, a.tp);
    match (bc1, bc2) {
        (Some(_), Some(bc2)) => card::get_attack(d, bc2) > 0,
        _ => false,
    }
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    let (b1, b2) = duel::get_battle_monster(d, a.tp);
    // if bc1:IsRelateToBattle() and bc1:IsFaceup() and bc2:IsRelateToBattle() and bc2:IsFaceup() then
    let bc1 = b1.nil();
    if card::is_relate_to_battle(d, bc1) && card::is_faceup(d, bc1) {
        let bc2 = b2.nil();
        if card::is_relate_to_battle(d, bc2) && card::is_faceup(d, bc2) {
            // bc1:UpdateAttack(bc2:GetAttack(),RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL,e:GetHandler())
            let amt = card::get_attack(d, bc2) as i64;
            let c = effect::get_handler(d, a.e).nil();
            card::update_attack(d, bc1, amt, Some(RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL), Some(c), None);
        }
    }
}
