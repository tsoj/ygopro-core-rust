//! Traptrix Atrax
use super::prelude::*;

pub const ID: u32 = 55428242;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_TRAP_HOLE,SET_HOLE}
    listed_series: &[SET_TRAP_HOLE, SET_HOLE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Unaffected by the effects of "Hole" Normal Traps
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(function(_,te) return s.is_normal_hole(te:GetHandler()) end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(1)).nil();
        is_normal_hole(d, h)
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can activate "Hole" Normal Traps from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetTarget(function(_,c) return s.is_normal_hole(c) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| is_normal_hole(d, c))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot negate the activation of your Normal Traps
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_INACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.chainfilter)
    effect::set_value_fn(d, e3, lua_fn(chainfilter));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot negate the effects of your Normal Traps
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e4, EFFECT_CANNOT_DISEFFECT);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetValue(s.chainfilter)
    effect::set_value_fn(d, e4, lua_fn(chainfilter));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CANNOT_DISABLE)
    effect::set_code(d, e5, EFFECT_CANNOT_DISABLE);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e5, LOCATION_ONFIELD, 0);
    // e5:SetTarget(s.distarget)
    effect::set_target(d, e5, Some(effect_card_fn(|d, _e, c| distarget(d, c))));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.is_normal_hole(c)`: `c:IsNormalTrap() and c:IsSetCard({SET_TRAP_HOLE,SET_HOLE})`
fn is_normal_hole(d: &mut Duel, c: CardId) -> bool {
    card::is_normal_trap(d, c) && card::is_set_card(d, c, &[SET_TRAP_HOLE as u16, SET_HOLE as u16])
}
/// `s.distarget(e,c)`: `c:IsNormalTrap() or c:IsCompositeType(TYPE_TRAP|TYPE_EQUIP)`
fn distarget(d: &mut Duel, c: CardId) -> bool {
    card::is_normal_trap(d, c) || card::is_composite_type(d, c, TYPE_TRAP | TYPE_EQUIP)
}
/// `s.chainfilter(e,ct)`: `Chain.IsTriggeringControler(ct,e:GetHandlerPlayer()) and Chain.IsTriggeringExactType(ct,TYPE_TRAP)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let ct = a.int(1) as u8;
    let p = effect::get_handler_player(d, e);
    chain::is_triggering(d, ct, chain::Prop::Controler, &[p as i64]).truthy() && chain::is_triggering_exact_type(d, ct, TYPE_TRAP).truthy()
}
