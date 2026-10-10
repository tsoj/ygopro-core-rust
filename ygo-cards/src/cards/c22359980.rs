//! Mirror Wall
use super::prelude::*;

pub const ID: u32 = 22359980;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_DAMAGE_STEP));
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    //
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkchange
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(effect_card_fn(atktg)));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e1:SetLabelObject(g)
    // e2:SetLabelObject(g)
    // e3:SetLabelObject(g)
    // (the group lives in e1; e2 and e3 refer to e1, as they share the Lua table g)
    effect::set_label_object(d, e1, Value::Group(CardSet::new()));
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_label_object(d, e3, Value::Effect(e1));
    // maintain
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.mtcon)
    effect::set_condition(d, e4, Some(condition(mtcon)));
    // e4:SetOperation(s.mtop)
    effect::set_operation(d, e4, Some(operation(mtop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// The effect whose label object is the group of this wall: e1 holds it, the others refer to e1.
fn wall_owner(d: &Duel, e: EffectId) -> EffectId {
    effect::get_label_object(d, e).as_effect().unwrap_or(e)
}
fn wall_group(d: &Duel, owner: EffectId) -> CardSet {
    effect::get_label_object(d, owner).as_group().cloned().nil()
}
fn set_wall_group(d: &mut Duel, owner: EffectId, g: CardSet) {
    effect::set_label_object(d, owner, Value::Group(g));
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetLabelObject():Clear()
    let owner = wall_owner(d, a.e);
    let mut g = wall_group(d, owner);
    g.clear();
    set_wall_group(d, owner, g);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if a and a:IsControler(1-tp) and a:IsFaceup() and a:IsLocation(LOCATION_MZONE) then
    if let Some(at) = at {
        if card::is_controler(d, at, 1 - a.tp) && card::is_faceup(d, at) && card::is_location(d, at, LOCATION_MZONE) {
            // e:GetLabelObject():AddCard(a)
            let owner = wall_owner(d, a.e);
            let mut g = wall_group(d, owner);
            group::add_card(&mut g, &CardSet::from_card(at));
            set_wall_group(d, owner, g);
            // if a:GetFlagEffect(id)==0 then
            if card::get_flag_effect(d, at, ID) == 0 {
                // a:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
                card::register_flag_effect(d, at, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            }
        }
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0 and e:GetLabelObject():IsContains(c)
    if card::get_flag_effect(d, c, ID) == 0 {
        return false;
    }
    let owner = wall_owner(d, e);
    group::is_contains(&wall_group(d, owner), c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetAttack()/2
    lua_int(card::get_attack(d, c) as f64 / 2.0)
}
/// `s.mtcon(e,tp,eg,ep,ev,re,r,rp)`
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.mtop(e,tp,eg,ep,ev,re,r,rp)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckLPCost(tp,2000) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::check_lp_cost(d, a.tp, 2000) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.PayLPCost(tp,2000)
        duel::pay_lp_cost(d, a.tp, 2000);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
