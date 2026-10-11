//! WINDS OF VICTORY
use super::prelude::*;

pub const ID: u32 = 37301660;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // The equipped monster gains 300 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Also it becomes WIND
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
    // e2:SetValue(ATTRIBUTE_WIND)
    effect::set_value(d, e2, ATTRIBUTE_WIND as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
