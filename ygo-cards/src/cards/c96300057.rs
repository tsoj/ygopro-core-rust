//! W-Wing Catapult
use super::prelude::*;

pub const ID: u32 = 96300057;

// s.listed_names={51638941}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[51638941], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,51638941))
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_code(d, c, &[51638941]))), false, None);
    // Atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(400)
    effect::set_value(d, e1, 400);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Def up
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
