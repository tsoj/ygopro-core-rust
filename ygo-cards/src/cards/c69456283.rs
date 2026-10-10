//! Koitsu
use super::prelude::*;

pub const ID: u32 = 69456283;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[48202661], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,48202661),true)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_code(d, cc, &[48202661]))), true, None);
    // atk up
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(3000)
    effect::set_value(d, e3, 3000);
    // e3:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e3, Some(aux::is_union_state()));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // pierce
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e4, EFFECT_PIERCE);
    // e4:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e4, Some(aux::is_union_state()));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
