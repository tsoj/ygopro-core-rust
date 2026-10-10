//! Power Bond
use super::prelude::*;

pub const ID: u32 = 37630732;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:RegisterEffect(Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE),nil,nil,nil,nil,s.stage2))
    let e = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_MACHINE))),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    card::register_effect(d, c, e, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk~=1 then return end
    if chk != 1 {
        return;
    }
    let tc = tc.nil();
    let h = effect::get_handler(d, e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(tc:GetBaseAttack())
    let base = card::get_base_attack(d, tc) as i64;
    effect::set_value(d, e1, base);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetLabel(tc:GetBaseAttack())
        effect::set_label(d, e2, &[base]);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // e2:SetOperation(s.damop)
        effect::set_operation(d, e2, Some(operation(damop)));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(tp,e:GetLabel(),REASON_EFFECT)
    let label = effect::get_label(d, a.e);
    duel::damage(d, a.tp, label, REASON_EFFECT, false, None);
}
