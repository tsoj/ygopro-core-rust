//! Metallizing Parasite - Lunatite
use super::prelude::*;

pub const ID: u32 = 7369217;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,nil,true)
    aux::add_union_procedure(d, c, None, true, None);
    // immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e1, Some(aux::is_union_state()));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efilter(e,te)`: `return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsSpellEffect()`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e) && effect::is_spell_effect(d, te)
}
