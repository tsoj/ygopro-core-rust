//! Bubble Blaster
use super::prelude::*;

pub const ID: u32 = 53586134;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[79979666], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,79979666))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_code(d, c, &[79979666]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // equip effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(800)
    effect::set_value(d, e3, 800);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e4:SetCode(EFFECT_DESTROY_SUBSTITUTE)
    effect::set_code(d, e4, EFFECT_DESTROY_SUBSTITUTE);
    // e4:SetValue(s.repval)
    effect::set_value_fn(d, e4, lua_fn(repval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
    // e5:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_AVOID_BATTLE_DAMAGE);
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.repval(e,re,r,rp)`
fn repval(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
