//! The Wicked Avatar
use super::prelude::*;

pub const ID: u32 = 21208154;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=aux.AddNormalSummonProcedure(c,true,false,3,3)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    // local e2=aux.AddNormalSetProcedure(c,true,false,3,3)
    let _e2 = aux::add_normal_set_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e4, EFFECT_SET_ATTACK_FINAL);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_REPEAT+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_REPEAT + EFFECT_FLAG_DELAY, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetValue(s.adval)
    effect::set_value_fn(d, e4, effect_value(adval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_SET_DEFENSE_FINAL)
    effect::set_code(d, e5, EFFECT_SET_DEFENSE_FINAL);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e6, EVENT_SUMMON_SUCCESS);
    // e6:SetOperation(s.regop)
    effect::set_operation(d, e6, Some(operation(regop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetCode(id)
    effect::set_code(d, e7, ID);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsHasEffect(id)
    card::is_faceup(d, c) && card::is_has_effect(d, c, ID, None).is_empty()
}
/// `s.adval(e,c)`
fn adval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=Duel.GetMatchingGroup(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,nil)
    let mut g = duel::get_matching_group(d, Some(&filter), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then
    if g.is_empty() {
        // return 100
        return 100;
    }
    // local tg,val=g:GetMaxGroup(Card.GetAttack)
    let (tg, val) = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).nil();
    // if not tg:IsExists(aux.TRUE,1,e:GetHandler()) then
    let h = effect::get_handler(d, e).nil();
    if !group::is_exists(d, &tg, &|_, _| true, 1, h.into()) {
        // g:RemoveCard(e:GetHandler())
        group::remove_card(&mut g, &CardSet::from_card(h));
        // tg,val=g:GetMaxGroup(Card.GetAttack)
        let (_, val) = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).nil();
        // return val+100
        return val + 100;
    }
    // return val+100
    val + 100
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.turncon)
    effect::set_condition(d, e2, Some(condition(turncon)));
    // e2:SetOperation(s.turnop)
    effect::set_operation(d, e2, Some(operation(turnop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local descnum=tp==c:GetOwner() and 0 or 1
    let descnum = if a.tp == card::get_owner(d, c) { 0 } else { 1 };
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetDescription(aux.Stringid(id,descnum))
    effect::set_description(d, e3, aux::stringid(ID, descnum));
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetCode(1082946)
    effect::set_code(d, e3, 1082946);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // e3:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e3, a.tp);
    // e3:SetOperation(s.reset)
    effect::set_operation(d, e3, Some(operation(reset)));
    // e3:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)`
fn reset(d: &mut Duel, a: &EffArgs) {
    // s.turnop(e:GetLabelObject(),tp,eg,ep,ev,e,r,rp)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    turnop_call(d, lo, a.tp, a.eg.clone(), a.ep, a.ev, Some(a.e), a.r, a.rp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.turncon(e,tp,eg,ep,ev,re,r,rp)`
fn turncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.turnop(e,tp,eg,ep,ev,re,r,rp)`
fn turnop(d: &mut Duel, a: &EffArgs) {
    turnop_call(d, a.e, a.tp, a.eg.clone(), a.ep, a.ev, a.re, a.r, a.rp);
}
/// The body of `s.turnop`, called with the effect and the arguments of the call
#[allow(clippy::too_many_arguments)]
fn turnop_call(d: &mut Duel, e: EffectId, _tp: u8, _eg: Option<CardSet>, _ep: u8, _ev: u32, re: Option<EffectId>, _r: u32, _rp: u8) {
    // local ct=e:GetLabel()+1
    let ct = effect::get_label(d, e) + 1;
    // e:GetHandler():SetTurnCounter(ct)
    let h = effect::get_handler(d, e).nil();
    card::set_turn_counter(d, h, ct as u16);
    // if ct==2 then
    if ct == 2 {
        // e:GetLabelObject():Reset()
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::reset(d, lo);
        // if re then re:Reset() end
        if let Some(re) = re {
            effect::reset(d, re);
        }
    } else {
        // e:SetLabel(ct)
        effect::set_label(d, e, &[ct]);
    }
}
