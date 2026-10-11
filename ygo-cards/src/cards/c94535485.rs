//! Hand-Holding Genie
use super::prelude::*;

pub const ID: u32 = 94535485;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atlimit)
    effect::set_value_fn(d, e1, effect_card_fn(atlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // defup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(s.defval)
    effect::set_value_fn(d, e2, effect_card_value(defval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    c != h
}
/// `s.deffilter(c)`
fn deffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetBaseDefense()>=0 and c:IsPosition(POS_FACEUP_DEFENSE)
    card::get_base_defense(d, c) >= 0 && card::is_position(d, c, POS_FACEUP_DEFENSE)
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=Duel.GetMatchingGroup(s.deffilter,c:GetControler(),LOCATION_MZONE,0,c)
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&deffilter), p, LOCATION_MZONE, 0, c.into());
    // return g:GetSum(Card.GetBaseDefense)
    group::get_sum(d, &g, &|d, x| card::get_base_defense(d, x) as i64)
}
