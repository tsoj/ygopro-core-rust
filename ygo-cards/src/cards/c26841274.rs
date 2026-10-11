//! D-Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 26841274;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DESTINY_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon 1 "Destiny HERO" fusion monster
    // Using monsters you control as fusion material
    // c:RegisterEffect(Fusion.CreateSummonEff(c,nil,Fusion.OnFieldMat(aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DESTINY_HERO)),nil,nil,nil,s.stage2))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            // Fusion.OnFieldMat(aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DESTINY_HERO)): the filter gets (c, e, tp, chk)
            // and calls Card.IsSetCard(c, SET_DESTINY_HERO, e, tp, chk): e is no card (nil), tp is the sumtype, chk the player
            matfilter: Some(fusion::on_field_mat_with(Arc::new(|d, c, _e, tp, chk| {
                card::is_set_card_ctx(d, c, &[SET_DESTINY_HERO as u16], None, tp as u64, Some(chk as u8))
            }))),
            stage2: Some(Arc::new(stage2)),
            ..Default::default()
        },
    );
    card::register_effect(d, c, e1, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk == 0 {
        // Cannot be destroyed by battle or card effect
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3008)
        effect::set_description(d, e1, 3008);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        let tc = tc.nil();
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
