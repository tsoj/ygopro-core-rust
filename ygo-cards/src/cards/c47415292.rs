//! Pitch-Dark Dragon
use super::prelude::*;

pub const ID: u32 = 47415292;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[11321183], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,11321183),true)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_code(d, cc, &[11321183]))), true, None);
    // atk/def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(400)
    effect::set_value(d, e3, 400);
    // e3:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e3, Some(aux::is_union_state()));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // pierce
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
    // e5:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e5, EFFECT_PIERCE);
    // e5:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e5, Some(aux::is_union_state()));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
