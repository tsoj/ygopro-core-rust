//! High Ritual Art
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 36350300;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,location=LOCATION_DECK,matfilter=s.mfilter,stage2=s.stage2})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            location: Some(LOCATION_DECK),
            matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, mc: CardId, _e: EffectId, _tp: u8| mfilter(d, mc)))),
            stage2: Some(Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_HAND) and c:IsType(TYPE_NORMAL)
    card::is_location(d, c, LOCATION_HAND) && card::is_type(d, c, TYPE_NORMAL)
}
/// s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, tc: CardId) {
    let (e, tp) = (a.e, a.tp);
    // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
    card::register_flag_effect(
        d,
        tc,
        ID,
        RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN,
        EFFECT_FLAG_CLIENT_HINT,
        1,
        0,
        aux::stringid(ID, 1),
    );
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    // e1:SetCondition(s.tdcon)
    effect::set_condition(d, e1, Some(condition(tdcon)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// s.tdcon(e,tp,eg,ep,ev,re,r,rp)
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // return Duel.IsTurnPlayer(1-tp) and e:GetLabelObject():GetFlagEffect(id)>0
    let tc = effect::get_label_object(d, e).as_card().nil();
    duel::is_turn_player(d, 1 - tp) && card::get_flag_effect(d, tc, ID) > 0
}
/// s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, e).as_card().nil();
    // Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, sc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
