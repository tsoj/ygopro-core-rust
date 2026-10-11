//! Thorn of Malice
use super::prelude::*;

pub const ID: u32 = 65079854;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLACK_ROSE_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))), None, None, None, None, None, None);
    // Atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(600)
    effect::set_value(d, e2, 600);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Pierce
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // indes
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_BATTLE);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    // e5:SetTarget(s.indestg)
    effect::set_target(d, e5, Some(effect_card_fn(indestg)));
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // battle
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCode(EVENT_BATTLED)
    effect::set_code(d, e6, EVENT_BATTLED);
    // e6:SetCondition(s.adcon)
    effect::set_condition(d, e6, Some(condition(adcon)));
    // e6:SetOperation(s.adop)
    effect::set_operation(d, e6, Some(operation(adop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_BLACK_ROSE_DRAGON]) || card::is_race(d, c, RACE_PLANT)
}
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetEquipTarget() and c==e:GetHandler():GetEquipTarget():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    match card::get_equip_target(d, h) {
        Some(eqt) => card::get_battle_target(d, eqt) == Some(c),
        None => false,
    }
}
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler():GetEquipTarget() and Duel.GetAttackTarget()~=nil
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == card::get_equip_target(d, h) && duel::get_attack_target(d).is_some()
}
fn adop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(-600)
    effect::set_value(d, e1, -600);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // bc:RegisterEffect(e2)
    card::register_effect(d, bc, e2, false);
}
