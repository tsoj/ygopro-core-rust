//! Doggy Diver
//! トラブル・ダイバー
use super::prelude::*;

pub const ID: u32 = 1003028;

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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| spcon(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // xyzlimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetValue(s.xyzlimit)
    effect::set_value_fn(d, e2, lua_fn(|d, a| xyzlimit(d, a)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args<'_>) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>0
    //   and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    //   and g:IsExists(aux.FaceupFilter(Card.IsLevel,4),1,nil)
    //   and not g:IsExists(aux.NOT(aux.FaceupFilter(Card.IsLevel,4)),1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) > 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && group::is_exists(d, &g, &level4_faceup, 1, NO_EX)
        && !group::is_exists(d, &g, &|d: &mut Duel, c| !level4_faceup(d, c), 1, NO_EX)
}
// aux.FaceupFilter(Card.IsLevel,4)
fn level4_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_level(d, c, &[4])
}
// function s.xyzlimit(e,c)
fn xyzlimit(d: &mut Duel, a: Args<'_>) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else {
        return false;
    };
    // return not c:IsRace(RACE_WARRIOR)
    !card::is_race(d, c, RACE_WARRIOR)
}
