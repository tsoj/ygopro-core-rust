//! Confusion Chaff
use super::prelude::*;

pub const ID: u32 = 67630339;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage cal
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=0
        set_s_var(d, ID, 0, Value::Int(0));
        // s[1]=0
        set_s_var(d, ID, 1, Value::Int(0));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        // ge1:SetOperation(s.check)
        effect::set_operation(d, ge1, Some(operation(check)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=Effect.CreateEffect(c)
        let ge2 = effect::create_effect(d, c);
        // ge2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge2:SetCode(EVENT_ATTACK_DISABLED)
        effect::set_code(d, ge2, EVENT_ATTACK_DISABLED);
        // ge2:SetOperation(s.check2)
        effect::set_operation(d, ge2, Some(operation(check2)));
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
        // aux.AddValuesReset(function()
        aux::add_values_reset(d, |d: &mut Duel| {
            // s[0]=0
            set_s_var(d, ID, 0, Value::Int(0));
            // s[1]=0
            set_s_var(d, ID, 1, Value::Int(0));
        });
    });
}
// s[i] read as a number (the global check sets s[0] and s[1] to 0)
fn get_s(d: &Duel, i: u64) -> i64 {
    s_var(d, ID, i).int()
}
// function s.check(e,tp,eg,ep,ev,re,r,rp)
fn check(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &eg);
    // if Duel.GetAttackTarget()==nil then
    if duel::get_attack_target(d).is_none() {
        // s[1-tc:GetControler()]=s[1-tc:GetControler()]+1
        let k = (1 - card::get_controler(d, tc.nil())) as u64;
        let v = get_s(d, k) + 1;
        set_s_var(d, ID, k, Value::Int(v));
        // Duel.GetAttacker():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let att = duel::get_attacker(d).nil();
        card::register_flag_effect(d, att, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // if s[1-tc:GetControler()]==1 then
        if get_s(d, k) == 1 {
            // s[2]=Duel.GetAttacker()
            let att = duel::get_attacker(d).map_or(Value::Nil, Value::Card);
            set_s_var(d, ID, 2, att);
        }
    }
}
// function s.check2(e,tp,eg,ep,ev,re,r,rp)
fn check2(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &eg);
    // if tc:GetFlagEffect(id)~=0 and Duel.GetAttackTarget()~=nil then
    if card::get_flag_effect(d, tc.nil(), ID) != 0 && duel::get_attack_target(d).is_some() {
        // s[1-tc:GetControler()]=s[1-tc:GetControler()]-1
        let k = (1 - card::get_controler(d, tc.nil())) as u64;
        let v = get_s(d, k) - 1;
        set_s_var(d, ID, k, Value::Int(v));
    }
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetAttackTarget()==nil and s[tp]==2
    //     and s[2]:GetFlagEffect(id)~=0 and Duel.GetAttacker()~=s[2]
    let s2 = s_var(d, ID, 2).as_card();
    duel::is_turn_player(d, 1 - tp)
        && duel::get_attack_target(d).is_none()
        && get_s(d, tp as u64) == 2
        && card::get_flag_effect(d, s2.nil(), ID) != 0
        && duel::get_attacker(d) != s2
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=s[2]
    let dc = s_var(d, ID, 2).as_card();
    // if a:GetFlagEffect(id)~=0 and d:GetFlagEffect(id)~=0
    //     and a:CanAttack() and not a:IsImmuneToEffect(e) and not d:IsImmuneToEffect(e) then
    if card::get_flag_effect(d, att.nil(), ID) != 0
        && card::get_flag_effect(d, dc.nil(), ID) != 0
        && card::can_attack(d, att.nil())
        && !card::is_immune_to_effect(d, att.nil(), a.e)
        && !card::is_immune_to_effect(d, dc.nil(), a.e)
    {
        // Duel.CalculateDamage(a,d)
        duel::calculate_damage(d, att.nil(), Some(dc.nil()), false);
    }
}
