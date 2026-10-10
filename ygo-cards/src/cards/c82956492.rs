//! D/D/D Oracle King d'Arc
use super::prelude::*;

pub const ID: u32 = 82956492;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DD],
    material_setcode: &[SET_DD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DD),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DD as u16], Some(fc), st, Some(tp)))),
            2,
        )],
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // damage conversion
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_REVERSE_DAMAGE)
    effect::set_code(d, e2, EFFECT_REVERSE_DAMAGE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(s.rev)
    effect::set_value_fn(d, e2, lua_fn(rev));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rev(e,re,r,rp,rc)`
fn rev(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_EFFECT)~=0
    (a.int(2) as u32 & REASON_EFFECT) != 0
}
