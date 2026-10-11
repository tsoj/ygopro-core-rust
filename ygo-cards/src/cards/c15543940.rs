//! Tyrant Dino Fusion
use super::prelude::*;

pub const ID: u32 = 15543940;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOWRESTLER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_DINOWRESTLER),Fusion.OnFieldMat,nil,nil,nil,s.stage2)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DINOWRESTLER as u16]))),
            matfilter: Some(fusion::on_field_mat()),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.stage2(e,tc,tp,mg,chk)
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk != 1 {
        return;
    }
    let tc = tc.nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // e1:SetValue(s.valcon)
    effect::set_value_fn(d, e1, lua_fn(valcon));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
}
// function s.valcon(e,re,r,rp)
fn valcon(d: &mut Duel, a: Args) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e(0)).as_card().nil();
    // if tc:GetFlagEffect(id)==0 then
    if card::get_flag_effect(d, tc, ID) == 0 {
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        // return r&REASON_BATTLE+REASON_EFFECT~=0
        let r = a.int(2) as u32;
        (r & (REASON_BATTLE + REASON_EFFECT)) != 0
    } else {
        // return false
        false
    }
}
