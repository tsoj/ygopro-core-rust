//! Dai Dance
use super::prelude::*;

pub const ID: u32 = 50696588;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make the opponent use only 1 Main Monster Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,nil,LOCATION_REASON_COUNT)>0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, tp, ID) == 0
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, Some(LOCATION_REASON_COUNT), None).0 > 0;
    }
    // local zone=Duel.SelectDisableField(tp,1,0,LOCATION_MZONE,0)
    let zone = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(0), false);
    // Duel.Hint(HINT_ZONE,tp,zone)
    duel::hint(d, HINT_ZONE as u8, tp, zone as u64);
    // e:SetLabel(zone>>16)
    effect::set_label(d, e, &[(zone >> 16) as i64]);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetFlagEffect(tp,id)>0 then return end
    if duel::get_flag_effect(d, tp, ID) > 0 {
        return;
    }
    // local zone=e:GetLabel()
    let zone = effect::get_label(d, e);
    // local nseq=math.log(zone,2)
    let nseq = (zone as f64).log2();
    // if not Duel.CheckLocation(1-tp,LOCATION_MZONE,nseq) then return end
    if !duel::check_location(d, 1 - tp, LOCATION_MZONE, nseq as u32) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,0,0,0)
    duel::register_flag_effect(d, tp, ID, 0, 0, 0, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, effect::get_handler(d, e).nil());
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_FORCE_MZONE)
    effect::set_code(d, e1, EFFECT_FORCE_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(zone|ZONES_EMZ)
    effect::set_value(d, e1, (zone as u32 | ZONES_EMZ) as i64);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, effect::get_handler(d, e).nil());
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ADJUST)
    effect::set_code(d, e2, EVENT_ADJUST);
    // e2:SetOperation(s.adjop)
    effect::set_operation(d, e2, Some(operation(adjop)));
    // e2:SetLabel(nseq)
    effect::set_label(d, e2, &[nseq as i64]);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
// function s.adjop(e,tp,eg,ep,ev,re,r,rp)
fn adjop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local nseq=e:GetLabel()
    let nseq = effect::get_label(d, e);
    // if not Duel.CheckLocation(1-tp,LOCATION_MZONE,nseq) then
    if !duel::check_location(d, 1 - tp, LOCATION_MZONE, nseq as u32) {
        // e:GetLabelObject():Reset()
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::reset(d, lo);
        // e:Reset()
        effect::reset(d, e);
    }
}
