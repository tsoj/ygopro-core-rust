//! Solemn Authority
//! 神の威光
use super::prelude::*;

pub const ID: u32 = 16308000;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AESIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,0,aux.FaceupFilter(Card.IsSetCard,SET_AESIR),nil,nil,TIMINGS_CHECK_MONSTER,TIMINGS_CHECK_MONSTER|TIMING_DRAW_PHASE,nil,nil,nil,s.operation)
    aux::add_persistent_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AESIR as u16]))),
        None,
        None,
        Some(TIMINGS_CHECK_MONSTER),
        Some(TIMINGS_CHECK_MONSTER | TIMING_DRAW_PHASE),
        None,
        None,
        None,
        Some(operation(op)),
        false,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
                effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
                // e2:SetRange(LOCATION_SZONE)
                effect::set_range(d, e2, LOCATION_SZONE);
                // e2:SetCountLimit(1)
                effect::set_count_limit(d, e2, 1, 0, 0, 0);
                // e2:SetLabel(2)
                effect::set_label(d, e2, &[2]);
                // e2:SetLabelObject(tc)
                effect::set_label_object(d, e2, Value::Card(tc));
                // e2:SetCondition(s.tgcon)
                effect::set_condition(d, e2, Some(condition(tgcon)));
                // e2:SetOperation(s.tgop)
                effect::set_operation(d, e2, Some(operation(tgop)));
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,2)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
                // c:RegisterEffect(e2)
                card::register_effect(d, c, e2, false);
            }
        }
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let mut ct = effect::get_label(d, a.e);
    // ct=ct-1
    ct -= 1;
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    // if ct==0 and e:GetHandler():IsHasCardTarget(e:GetLabelObject()) then
    if ct == 0 {
        let h = effect::get_handler(d, a.e).nil();
        let tc = effect::get_label_object(d, a.e).as_card().nil();
        if card::is_has_card_target(d, h, tc) {
            // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
            duel::send_to_grave(d, h, REASON_EFFECT, None, None);
        }
    }
}
