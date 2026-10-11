//! Gaia Drake, the Universal Force
use super::prelude::*;

pub const ID: u32 = 58601383;

pub const INFO: ScriptInfo = ScriptInfo { miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,97204936,s.ffilter)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(97204936), fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp)))]);
    // cannot be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.efilter1)
    effect::set_value_fn(d, e2, lua_fn(efilter1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot be destroyed
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.efilter2)
    effect::set_value_fn(d, e3, lua_fn(efilter2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_SYNCHRO,fc,sumtype,tp) and not c:IsType(TYPE_EFFECT,fc,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(fc), sumtype, Some(tp)) && !card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), sumtype, Some(tp))
}
/// `s.efilter1(e,re,rp)`
fn efilter1(d: &mut Duel, a: Args) -> bool {
    // return re:IsActiveType(TYPE_EFFECT)
    effect::is_active_type(d, a.e(1), TYPE_EFFECT)
}
/// `s.efilter2(e,re)`
fn efilter2(d: &mut Duel, a: Args) -> bool {
    // return re:IsActiveType(TYPE_EFFECT)
    effect::is_active_type(d, a.e(1), TYPE_EFFECT)
}
