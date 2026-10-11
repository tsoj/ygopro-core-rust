//! Rhinotaurus
use super::prelude::*;

pub const ID: u32 = 83957459;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Second attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetCondition(s.macon)
    effect::set_condition(d, e1, Some(condition(macon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=0
        // s[1]=0
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, ge1, EVENT_BATTLE_DESTROYING);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function()
        //     s[0]=0
        //     s[1]=0
        // end)
        aux::add_values_reset(d, |d| {
            set_s_var(d, ID, 0, Value::Int(0));
            set_s_var(d, ID, 1, Value::Int(0));
        });
    });
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local bc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let bc = group::get_first(d, &eg).nil();
    // local cp=bc:GetControler()
    let mut cp = card::get_controler(d, bc);
    // if not bc:IsRelateToBattle() then cp=bc:GetPreviousControler() end
    if !card::is_relate_to_battle(d, bc) {
        cp = card::get_previous_controler(d, bc);
    }
    // s[cp]=s[cp]+1
    let v = s_var(d, ID, cp as u64).int();
    set_s_var(d, ID, cp as u64, Value::Int(v + 1));
}
// function s.macon(e)
// return s[e:GetHandlerPlayer()]>=2
fn macon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    s_var(d, ID, p as u64).int() >= 2
}
