//! Swift Gaia the Fierce Knight
use super::prelude::*;

pub const ID: u32 = 16589042;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4 and Duel.GetFieldGroupCount(c:GetControler(),LOCATION_HAND,0)==1
    //     and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let minc = a.int(2);
    minc == 0 && card::get_level(d, c) > 4 && {
        let p = card::get_controler(d, c);
        duel::get_field_group_count(d, p, LOCATION_HAND, 0) == 1 && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
