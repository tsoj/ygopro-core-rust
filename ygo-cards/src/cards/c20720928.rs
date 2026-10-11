//! Metaphys Factor
use super::prelude::*;

pub const ID: u32 = 20720928;

// s.listed_series={SET_METAPHYS}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METAPHYS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // decrease tribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.ntcon)
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
    // e2:SetTarget(aux.FieldSummonProcTg(s.nttg))
    effect::set_target(d, e2, Some(aux::field_summon_proc_tg(Some(effect_card_fn(nttg)), None)));
    // e2:SetOperation(s.ntop)
    effect::set_operation(d, e2, Some(lua_fn(ntop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // act limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetOperation(s.chainop)
    effect::set_operation(d, e3, Some(operation(chainop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ntcon(e,c,minc)
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    if a.int(2) != 0 {
        return false;
    }
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
// function s.nttg(e,c)
fn nttg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsSetCard(SET_METAPHYS)
    card::is_level_above(d, c, 5) && card::is_set_card(d, c, &[SET_METAPHYS as u16])
}
// function s.ntop(e,tp,eg,ep,ev,re,r,rp,c)
fn ntop(d: &mut Duel, a: Args) -> Value {
    let (e, tp, c) = (a.e(0), a.p(1), a.c(8));
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
    card::register_flag_effect(
        d,
        c,
        ID,
        RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD),
        EFFECT_FLAG_CLIENT_HINT,
        1,
        0,
        aux::stringid(ID, 1),
    );
    // local e2=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetLabel(Duel.GetTurnCount()+1)
    let turn = duel::get_turn_count(d, None) as i64 + 1;
    effect::set_label(d, e2, &[turn]);
    // e2:SetLabelObject(c)
    effect::set_label_object(d, e2, Value::Card(c));
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // e2:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    Value::Nil
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, tc, ID) != 0 {
        // return Duel.GetTurnCount()==e:GetLabel()
        let turn = duel::get_turn_count(d, None) as i64;
        turn == effect::get_label(d, a.e)
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
// function s.chainop(e,tp,eg,ep,ev,re,r,rp)
fn chainop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if re:GetHandler():IsSetCard(SET_METAPHYS) and re:IsMonsterEffect() and re:GetOwnerPlayer()==tp then
    let rh = effect::get_handler(d, re).nil();
    if card::is_set_card(d, rh, &[SET_METAPHYS as u16]) && effect::is_monster_effect(d, re) && effect::get_owner_player(d, re) == a.tp {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
// function s.chainlm(e,rp,tp)
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
