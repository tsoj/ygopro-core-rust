//! Stellarknight Alpha
use super::prelude::*;

pub const ID: u32 = 81810441;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,0,aux.FilterBoolFunction(Card.IsSetCard,SET_TELLARKNIGHT),s.eqlimit)
    aux::add_equip_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]))),
        Some(lua_fn(eqlimit)),
        None,
        None,
        None,
        None,
        None,
    );
    // atk/def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(500)
    effect::set_value(d, e3, 500);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // immune
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
    // e5:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e5, EFFECT_IMMUNE_EFFECT);
    // e5:SetValue(s.efilter)
    effect::set_value_fn(d, e5, lua_fn(efilter));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // selfdes
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e6, EFFECT_SELF_DESTROY);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.descon)
    effect::set_condition(d, e6, Some(condition(descon)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsSetCard(SET_TELLARKNIGHT) and c:GetControler()==e:GetHandler():GetControler()
    let e = a.e(0);
    let c = a.c(1);
    let h = effect::get_handler(d, e).nil();
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && card::get_controler(d, c) == card::get_controler(d, h)
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandlerPlayer()~=re:GetOwnerPlayer()
    let e = a.e(0);
    let re = a.e(1);
    effect::get_handler_player(d, e) != effect::get_owner_player(d, re)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsFaceup() and not c:IsSetCard(SET_TELLARKNIGHT)
    card::is_faceup(d, c) && !card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16])
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
