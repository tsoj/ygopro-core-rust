//! Invoked Cocytus
use super::prelude::*;

pub const ID: u32 = 85908279;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,86120751,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(86120751),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_WATER, Some(fc), st, Some(tp)))),
        ],
    );
    // cannot target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.indval)
    effect::set_value_fn(d, e2, lua_fn(indval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // defense attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DEFENSE_ATTACK)
    effect::set_code(d, e3, EFFECT_DEFENSE_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.indval(e,re,tp)`
fn indval(d: &mut Duel, a: Args) -> bool {
    // return tp~=e:GetHandlerPlayer()
    let e = a.e(0);
    let tp = a.p(2);
    tp != effect::get_handler_player(d, e)
}
