//! TA.I. Strike
use super::prelude::*;

pub const ID: u32 = 86449372;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_IGNISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    // if not bc then return false end
    let Some(mut bc) = duel::get_attack_target(d) else {
        return false;
    };
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d).nil();
    // if not tc:IsControler(tp) then tc,bc=bc,tc end
    if !card::is_controler(d, tc, a.tp) {
        std::mem::swap(&mut tc, &mut bc);
    }
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsControler(tp) and not bc:IsControler(tp)
    card::is_controler(d, tc, a.tp) && !card::is_controler(d, bc, a.tp)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc);
    // if tc:IsRelateToBattle() and not tc:IsImmuneToEffect(e)
    //     and tc:IsControler(tp) and not bc:IsControler(tp) then
    if card::is_relate_to_battle(d, tc)
        && !card::is_immune_to_effect(d, tc, a.e)
        && card::is_controler(d, tc, a.tp)
        && !card::is_controler(d, bc.nil(), a.tp)
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(bc:GetAttack())
        let batk = card::get_attack(d, bc.nil()) as i64;
        effect::set_value(d, e1, batk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
        // e2:SetOperation(s.damop)
        effect::set_operation(d, e2, Some(operation(damop)));
        // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.damage(c)`
fn damage(d: &mut Duel, c: CardId) {
    // if c:GetReason()&0x21==0x21 then
    if (card::get_reason(d, c) & 0x21) == 0x21 {
        // Duel.Damage(c:GetPreviousControler(),c:GetBaseAttack(),REASON_EFFECT)
        let p = card::get_previous_controler(d, c);
        let atk = card::get_base_attack(d, c);
        duel::damage(d, p, atk as i64, REASON_EFFECT, false, None);
    }
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // s.damage(Duel.GetAttacker())
    let atk = duel::get_attacker(d).nil();
    damage(d, atk);
    // s.damage(Duel.GetAttackTarget())
    let at = duel::get_attack_target(d).nil();
    damage(d, at);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_IGNISTER) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsControler(tp) and c:IsReason(REASON_BATTLE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_IGNISTER as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return e:GetHandler():IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp)
        if !card::is_able_to_remove(d, h, None, None, None) {
            return false;
        }
        let eg = a.eg.clone().nil();
        let tp = a.tp;
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
