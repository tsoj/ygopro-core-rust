//! Raregold Armor
use super::prelude::*;

pub const ID: u32 = 7625614;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // atk target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetValue(s.atktg)
    effect::set_value_fn(d, e2, effect_card_fn(atktg));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // return ec and ec:GetControler()==e:GetHandlerPlayer()
    match ec {
        Some(ec) => card::get_controler(d, ec) == effect::get_handler_player(d, a.e),
        None => false,
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) != Some(c)
}
