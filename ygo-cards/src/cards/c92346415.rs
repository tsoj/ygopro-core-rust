//! Psychic Sword
use super::prelude::*;

pub const ID: u32 = 92346415;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_PSYCHIC))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_PSYCHIC))), None, None, None, None, None, None);
    // Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local dif=Duel.GetLP(1-e:GetHandlerPlayer())-Duel.GetLP(e:GetHandlerPlayer())
    let dif = duel::get_lp(d, 1 - effect::get_handler_player(d, e)) as i64 - duel::get_lp(d, effect::get_handler_player(d, e)) as i64;
    // if dif>0 then return dif>2000 and 2000 or dif
    if dif > 0 {
        if dif > 2000 { 2000 } else { dif }
    } else {
        // else return 0 end
        0
    }
}
