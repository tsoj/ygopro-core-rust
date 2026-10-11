//! Gogogo Talisman
use super::prelude::*;

pub const ID: u32 = 63583431;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOGOGO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // reduce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetValue(s.damval)
    effect::set_value_fn(d, e2, lua_fn(damval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e3, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // indes
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetHintTiming(TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e4, TIMING_BATTLE_PHASE, None);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.indcon)
    effect::set_condition(d, e4, Some(condition(indcon)));
    // e4:SetTarget(s.indtg)
    effect::set_target(d, e4, Some(cost_target(indtg)));
    // e4:SetOperation(s.indop)
    effect::set_operation(d, e4, Some(operation(indop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GOGOGO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GOGOGO as u16])
}
// function s.damcon(e)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,2,nil)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&cfilter), hp, LOCATION_MZONE, 0, 2, NO_EX)
}
// function s.damval(e,re,val,r,rp,rc)
fn damval(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_EFFECT)~=0 then return 0 end
    if (a.int(3) & REASON_EFFECT as i64) != 0 {
        return 0;
    }
    // return val
    a.int(2)
}
// function s.indcon(e,tp,eg,ep,ev,re,r,rp)
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc:IsControler(1-tp) then tc=Duel.GetAttackTarget() end
    if card::is_controler(d, tc.nil(), 1 - tp) {
        tc = duel::get_attack_target(d);
    }
    // e:SetLabelObject(tc)
    let label = match tc {
        Some(t) => Value::Card(t),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    // return tc and tc:IsFaceup() and tc:IsControler(tp) and tc:IsSetCard(SET_GOGOGO)
    match tc {
        None => false,
        Some(t) => card::is_faceup(d, t) && card::is_controler(d, t, tp) && card::is_set_card(d, t, &[SET_GOGOGO as u16]),
    }
}
// function s.indtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn indtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(e:GetLabelObject())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::set_target_card(d, tc);
    true
}
// function s.indop(e,tp,eg,ep,ev,re,r,rp)
fn indop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
