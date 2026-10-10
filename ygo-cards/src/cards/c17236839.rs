//! Flash Fusion
use super::prelude::*;

pub const ID: u32 = 17236839;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,nil,Fusion.OnFieldMat,nil,nil,nil,s.stage2)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::on_field_mat()),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // local fid=e:GetHandler():GetFieldID()
        let h = effect::get_handler(d, e).nil();
        let fid = card::get_field_id(d, h);
        let tc = tc.nil();
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid as i64]);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::card(Some(tc)));
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() != Some(effect::get_label(d, e)) {
        // e:Reset()
        effect::reset(d, e);
        // return false
        false
    } else {
        // return true
        true
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetLabelObject(),REASON_EFFECT)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
}
