//! Legendary Six Samurai - Kizan
use super::prelude::*;

pub const ID: u32 = 49721904;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| spcon(d, a.e(0), a.card(1)))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk,def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.valcon)
    effect::set_condition(d, e2, Some(condition(valcon)));
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI) and c:GetCode()~=id
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && card::get_code(d, c)[0] != ID
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and
    //   Duel.IsExistingMatchingCard(s.spfilter,c:GetControler(),LOCATION_MZONE,0,1,nil)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.vfilter(c)`
fn vfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
}
/// `s.valcon(e)`
fn valcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsExistingMatchingCard(s.vfilter,c:GetControler(),LOCATION_MZONE,0,2,c)
    let tp = card::get_controler(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| vfilter(d, x)), tp, LOCATION_MZONE, 0, 2, c.into())
}
