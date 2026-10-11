//! Hammer Bounzer
use super::prelude::*;

pub const ID: u32 = 44790889;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with no tribute
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // actlimit
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.ntcon(e,c,minc)
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(c:GetControler(),LOCATION_ONFIELD,0,nil)==0
    //     and Duel.GetFieldGroupCount(c:GetControler(),0,LOCATION_ONFIELD,nil)>0
    if a.int(2) != 0 || card::get_level(d, c) <= 4 {
        return false;
    }
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) == 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()~=nil
    //     and not Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,1,nil)
    duel::get_attack_target(d).is_some()
        && !duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
