//! Superalloy Beast Raptinus
use super::prelude::*;

pub const ID: u32 = 64463828;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsType,TYPE_GEMINI),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_GEMINI, Some(fc), st, Some(tp)))), 2)],
    );
    // duel status
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsType,TYPE_GEMINI))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_GEMINI))));
    // e1:SetCode(EFFECT_GEMINI_STATUS)
    effect::set_code(d, e1, EFFECT_GEMINI_STATUS);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
