//! Mirror of Yata
use super::prelude::*;

pub const ID: u32 = 1200843;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_card_types: &[TYPE_SPIRIT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsType,TYPE_SPIRIT))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_type(d, x, TYPE_SPIRIT))), None, None, None, None, None, None);
    // The equipped monster don't have to return to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_SPIRIT_MAYNOT_RETURN)
    effect::set_code(d, e1, EFFECT_SPIRIT_MAYNOT_RETURN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destruction replacement for the equipped monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_DESTROY_SUBSTITUTE)
    effect::set_code(d, e2, EFFECT_DESTROY_SUBSTITUTE);
    // e2:SetValue(s.desval)
    effect::set_value_fn(d, e2, lua_fn(desval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.desval(e,re,r,rp)
fn desval(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
