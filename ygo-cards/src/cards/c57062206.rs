//! Doitsu
use super::prelude::*;

pub const ID: u32 = 57062206;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[60246171], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,60246171),true)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(unfilter)), true, None);
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(2500)
    effect::set_value(d, e1, 2500);
    // e1:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e1, Some(aux::is_union_state()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FilterBoolFunction(Card.IsCode,60246171)`
fn unfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[60246171])
}
