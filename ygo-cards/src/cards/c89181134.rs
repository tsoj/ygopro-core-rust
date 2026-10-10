//! Predaplant Chlamydosundew
use super::prelude::*;

pub const ID: u32 = 89181134;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_PREDATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusattribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.attrtg)
    effect::set_target(d, e1, Some(effect_card_fn(attrtg)));
    // e1:SetValue(s.attrval)
    effect::set_value_fn(d, e1, lua_fn(attrval));
    // e1:SetOperation(s.attrcon)
    effect::set_operation(d, e1, Some(lua_fn(attrcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // fusion summon
    // local params = {aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK),nil,s.fextra,nil,Fusion.ForcedHandler}
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    let p = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_attribute(d, c, ATTRIBUTE_DARK))),
        extrafil: Some(std::sync::Arc::new(fextra)),
        gc: Some(fusion::forced_handler()),
        ..Default::default()
    };
    // e2:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, p.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op = fusion::summon_eff_op(d, p);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.attrtg(e,c)`
fn attrtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetCounter(COUNTER_PREDATOR)>0
    card::get_counter(d, c, COUNTER_PREDATOR as u16) > 0
}
/// `s.attrval(e,c,rp)`
fn attrval(d: &mut Duel, a: Args) -> i64 {
    // if rp==e:GetHandlerPlayer() then
    let rp = a.p(2);
    if rp == effect::get_handler_player(d, a.e(0)) {
        // return ATTRIBUTE_DARK
        return ATTRIBUTE_DARK as i64;
    }
    // else return c:GetAttribute() end
    let c = a.c(1);
    card::get_attribute(d, c) as i64
}
/// `s.attrcon(scard,sumtype,tp)`
fn attrcon(_d: &mut Duel, a: Args) -> bool {
    // return (sumtype&MATERIAL_FUSION)~=0
    (a.int(1) & MATERIAL_FUSION as i64) != 0
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetCounter(COUNTER_PREDATOR)>0
    card::is_faceup(d, c) && card::get_counter(d, c, COUNTER_PREDATOR as u16) > 0
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(s.filter),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && filter(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    (Some(g), None)
}
