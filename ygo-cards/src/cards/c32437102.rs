//! Dragonic Attack
use super::prelude::*;

pub const ID: u32 = 32437102;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_WARRIOR),s.eqlimit)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_WARRIOR))),
        Some(lua_fn(eqlimit)),
        None,
        None,
        None,
        None,
        None,
    );
    // Atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // race
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e4, EFFECT_CHANGE_RACE);
    // e4:SetValue(RACE_DRAGON)
    effect::set_value(d, e4, RACE_DRAGON as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.card(1);
    // return e:GetHandler():GetEquipTarget()==c or c:IsRace(RACE_WARRIOR)
    let h = effect::get_handler(d, e).nil();
    if card::get_equip_target(d, h) == c {
        return true;
    }
    let c = c.nil();
    card::is_race(d, c, RACE_WARRIOR)
}
