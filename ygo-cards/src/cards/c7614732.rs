//! Greater Polymerization
use super::prelude::*;

pub const ID: u32 = 7614732;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion.RegisterSummonEff{handler=c,mincount=3,stage2=s.stage2}
    fusion::register_summon_eff(
        d,
        c,
        fusion::SummonEff {
            mincount: Some(3),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    if chk == 0 {
        let tc = tc.nil();
        // Piercing
        // local e1=Effect.CreateEffect(tc)
        let e1 = effect::create_effect(d, tc);
        // e1:SetDescription(3208)
        effect::set_description(d, e1, 3208);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e1, EFFECT_PIERCE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // Cannot be destroyed by card effects
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetDescription(3001)
        effect::set_description(d, e2, 3001);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc, e2, true);
        // if not tc:IsType(TYPE_EFFECT) then
        if !card::is_type(d, tc, TYPE_EFFECT) {
            // Becomes an Effect Monster if it wasn't already one
            // local e3=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e3 = effect::create_effect(d, h);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_ADD_TYPE)
            effect::set_code(d, e3, EFFECT_ADD_TYPE);
            // e3:SetValue(TYPE_EFFECT)
            effect::set_value(d, e3, TYPE_EFFECT as i64);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e3,true)
            card::register_effect(d, tc, e3, true);
        }
    }
}
