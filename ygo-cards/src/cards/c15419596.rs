//! Ghost Bird of Bewitchment
//! Script by nekrozar
use super::prelude::*;

pub const ID: u32 = 15419596;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(lua_fn(atkcon)));
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // extra attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK_MONSTER);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.eacon)
    effect::set_condition(d, e3, Some(lua_fn(eacon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.indcon)
    effect::set_condition(d, e4, Some(lua_fn(indcon)));
    // e4:SetValue(aux.tgoval)
    effect::set_value_fn(d, e4, aux::tgoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // indes
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.indcon)
    effect::set_condition(d, e5, Some(lua_fn(indcon)));
    // e5:SetValue(s.indval)
    effect::set_value_fn(d, e5, lua_fn(indval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // cannot activate
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e6, EFFECT_CANNOT_ACTIVATE);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(1,1)
    effect::set_target_range(d, e6, 1, 1);
    // e6:SetCondition(s.actcon)
    effect::set_condition(d, e6, Some(lua_fn(actcon)));
    // e6:SetValue(s.aclimit)
    effect::set_value_fn(d, e6, lua_fn(aclimit));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetSequence()==0
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_sequence(d, h) == 0
}
/// `s.eacon(e)`
fn eacon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetSequence()==4
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_sequence(d, h) == 4
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetSequence()==2
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_sequence(d, h) == 2
}
/// `s.indval(e,re,tp)`
fn indval(d: &mut Duel, a: Args) -> bool {
    // return tp~=e:GetHandlerPlayer()
    let e = a.e(0);
    a.p(2) != effect::get_handler_player(d, e)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:GetSequence()==1 or c:GetSequence()==3
    let seq = card::get_sequence(d, c);
    seq == 1 || seq == 3
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local tc=re:GetHandler()
    let re = a.e(1);
    let tc = effect::get_handler(d, re).nil();
    // return tc:IsLocation(LOCATION_MZONE) and re:IsMonsterEffect() and e:GetHandler():GetColumnGroup():IsContains(tc)
    card::is_location(d, tc, LOCATION_MZONE)
        && effect::is_monster_effect(d, re)
        && {
            let h = effect::get_handler(d, a.e(0)).nil();
            let g = card::get_column_group(d, h, 0, 0);
            group::is_contains(&g, tc)
        }
}
