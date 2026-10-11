//! Performapal Odd-Eyes Minitaurus
//! EMオッドアイズ・ミノタウロス
use super::prelude::*;

pub const ID: u32 = 10731333;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // pierce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.ptg)
    effect::set_target(d, e1, Some(effect_card_fn(ptg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ptg(e,c)`
fn ptg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_PERFORMAPAL) or c:IsSetCard(SET_ODD_EYES)
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) || card::is_set_card(d, c, &[SET_ODD_EYES as u16])
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsSetCard(SET_PERFORMAPAL) or c:IsSetCard(SET_ODD_EYES))
    card::is_faceup(d, c) && (card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) || card::is_set_card(d, c, &[SET_ODD_EYES as u16]))
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let bt = card::get_battle_target(d, att);
    // local gc=Duel.GetMatchingGroupCount(s.atkfilter,tp,LOCATION_ONFIELD,0,nil)
    let gc = duel::get_matching_group_count(d, Some(&atkfilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    // return a:IsControler(tp) and a:IsType(TYPE_PENDULUM) and d
    //     and d:IsFaceup() and not d:IsControler(tp) and gc>0
    if !(card::is_controler(d, att, tp) && card::is_type(d, att, TYPE_PENDULUM)) {
        return false;
    }
    match bt {
        Some(bt) => card::is_faceup(d, bt) && !card::is_controler(d, bt, tp) && gc > 0,
        None => false,
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local d=Duel.GetAttacker():GetBattleTarget()
    let att = duel::get_attacker(d).nil();
    let bt = card::get_battle_target(d, att).nil();
    // local gc=Duel.GetMatchingGroupCount(s.atkfilter,tp,LOCATION_ONFIELD,0,nil)
    let gc = duel::get_matching_group_count(d, Some(&atkfilter), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // if d:IsRelateToBattle() and d:IsFaceup() then
    if card::is_relate_to_battle(d, bt) && card::is_faceup(d, bt) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // e1:SetValue(-gc*100)
        effect::set_value(d, e1, -(gc as i64) * 100);
        // d:RegisterEffect(e1)
        card::register_effect(d, bt, e1, false);
    }
}
