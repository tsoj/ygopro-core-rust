//! Forbidden Arts of the Gishki
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 28429121;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GISHKI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Ritual.CreateProc(c,RITPROC_EQUAL,aux.FilterBoolFunction(Card.IsSetCard,SET_GISHKI),nil,nil,s.extrafil,s.extraop,aux.FilterBoolFunction(Card.IsOnField))
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_GISHKI as u16]))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            extraop: Some(Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, sc: CardId| extraop(d, mat, a, sc))),
            matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8| card::is_on_field(d, x)))),
            ..Default::default()
        },
    );
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_current_phase(d) != PHASE_MAIN2;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.mfilter(c,e)`
fn mfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c) && card::get_level(d, c) > 0 && !card::is_immune_to_effect(d, c, e) && card::is_releasable(d, c, None, None)
}
/// `s.extrafil(e,tp,eg,ep,ev,re,r,rp)`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    let (e, tp) = (a.e, a.tp);
    // return Duel.GetMatchingGroup(s.mfilter,tp,0,LOCATION_MZONE,nil,e)
    Some(duel::get_matching_group(d, Some(&|d, c| mfilter(d, c, e)), tp, 0, LOCATION_MZONE, NO_EX))
}
/// `s.extraop(mat,e,tp,eg,ep,ev,re,r,rp,sc)`
fn extraop(d: &mut Duel, mat: &CardSet, a: &EffArgs, sc: CardId) {
    // Duel.ReleaseRitualMaterial(mat)
    duel::release_ritual_material(d, mat, false);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(sc:GetAttack()/2)
    let atk = card::get_attack(d, sc);
    effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // sc:RegisterEffect(e1)
    card::register_effect(d, sc, e1, false);
}
