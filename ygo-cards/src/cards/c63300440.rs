//! Dragon Shield
use super::prelude::*;

pub const ID: u32 = 63300440;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_DRAGON))), None, None, None, None, None, None);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_NO_BATTLE_DAMAGE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e3:Clone()
    let e6 = effect::clone(d, e3);
    // e6:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e6, EFFECT_AVOID_BATTLE_DAMAGE);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
