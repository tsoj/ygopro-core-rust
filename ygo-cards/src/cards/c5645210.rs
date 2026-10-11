//! The splendid VENUS
use super::prelude::*;

pub const ID: u32 = 5645210;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk,def
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.NOT(aux.TargetBoolFunction(Card.IsRace,RACE_FAIRY)))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_FAIRY))));
    // e1:SetValue(-500)
    effect::set_value(d, e1, -500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.NOT(aux.TargetBoolFunction(Card.IsRace,RACE_FAIRY)))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_FAIRY))));
    // e1:SetValue(-500)
    effect::set_value(d, e1, -500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // inactivatable
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_INACTIVATE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetValue(s.effectfilter)
    effect::set_value_fn(d, e4, lua_fn(effectfilter));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e5, EFFECT_CANNOT_DISEFFECT);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetValue(s.effectfilter)
    effect::set_value_fn(d, e5, lua_fn(effectfilter));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.effectfilter(e,ct)`
fn effectfilter(d: &mut Duel, a: Args) -> bool {
    // local p=e:GetHandler():GetControler()
    let h = effect::get_handler(d, a.e(0)).nil();
    let p = card::get_controler(d, h);
    // local te,tp,loc=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.int(1) as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_LOCATION]);
    let te = info[0].as_effect().nil();
    let tp = info[1].int() as u8;
    let loc = info[2].int() as u32;
    // return p==tp and te:IsSpellTrapEffect() and (loc&LOCATION_ONFIELD)~=0
    p == tp && effect::is_spell_trap_effect(d, te) && (loc & LOCATION_ONFIELD) != 0
}
