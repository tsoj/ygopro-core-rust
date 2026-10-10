//! Bashing Shield
use super::prelude::*;

pub const ID: u32 = 88610708;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSummonType,SUMMON_TYPE_NORMAL))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_summon_type(d, c, &[SUMMON_TYPE_NORMAL as u32]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(lua_fn(damcon)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.damcon(e)`
fn damcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetEquipTarget():GetControler()==e:GetHandlerPlayer()
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    let t = card::get_equip_target(d, h).nil();
    card::get_controler(d, t) == effect::get_handler_player(d, e)
}
