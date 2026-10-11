//! Metalsilver Armor
use super::prelude::*;

pub const ID: u32 = 33114323;
const LOC_MZN_GRV_BAN: u32 = LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // --Limit effect target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOC_MZN_GRV_BAN,LOC_MZN_GRV_BAN)
    effect::set_target_range(d, e1, LOC_MZN_GRV_BAN, LOC_MZN_GRV_BAN);
    // e1:SetCondition(s.cond)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(effect_card_fn(efftg)));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cond(e)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return tc and tc:GetControler()==e:GetHandler():GetControler()
    match card::get_equip_target(d, h) {
        Some(tc) => card::get_controler(d, tc) == card::get_controler(d, h),
        None => false,
    }
}
/// `s.efftg(e,c)`
fn efftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler():GetEquipTarget() and c:IsMonster()
    //     and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED))
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) != Some(c)
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
}
