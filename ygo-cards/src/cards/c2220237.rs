//! Secure Gardna
use super::prelude::*;

pub const ID: u32 = 2220237;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, lc, st, tp| matfilter(d, c, lc, st, tp))), 1, Some(1), None, None, None);
    // Cannot be used as Link Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // After summoning, take no damage for the turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Each turn, reduce battle or effect damage to 0 the first time.
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_DAMAGE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(s.damval2)
    effect::set_value_fn(d, e3, lua_fn(damval2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e4, EFFECT_NO_EFFECT_DAMAGE);
    // e4:SetCondition(function(e)return e:GetHandler():GetFlagEffect(id)==0 end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_flag_effect(d, h, ID) == 0
    })));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.damval1)
    effect::set_value_fn(d, e1, lua_fn(damval1));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_CYBERSE,lc,sumtype,tp) and c:IsType(TYPE_LINK,lc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_CYBERSE, lc, sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_LINK, lc, sumtype, Some(tp))
}
/// `s.damval1(e,re,val,r,rp,rc)`
fn damval1(_d: &mut Duel, a: Args) -> i64 {
    let val = a.int(2);
    let r = a.int(3) as u32;
    // if r&REASON_EFFECT~=0 then return 0 else return val end
    if r & REASON_EFFECT != 0 { 0 } else { val }
}
/// `s.damval2(e,re,val,r,rp,rc)`
fn damval2(d: &mut Duel, a: Args) -> i64 {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    let val = a.int(2);
    let r = a.int(3) as u32;
    // if r&(REASON_BATTLE|REASON_EFFECT)~=0 and c:GetFlagEffect(id)==0 then
    if r & (REASON_BATTLE | REASON_EFFECT) != 0 && card::get_flag_effect(d, c, ID) == 0 {
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // return 0
        return 0;
    }
    // return val
    val
}
