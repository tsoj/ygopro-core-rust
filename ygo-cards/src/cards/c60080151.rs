//! Memory of an Adversary
//! 好敵手の記憶
use super::prelude::*;

pub const ID: u32 = 60080151;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // if chk==0 then return tc:IsOnField() and tc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_on_field(d, tc) && card::is_able_to_remove(d, tc, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,tc:GetAttack())
    let atk = card::get_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, atk);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetAttacker()
    // if tc and tc:CanAttack() and tc:IsRelateToBattle() and not tc:IsStatus(STATUS_ATTACK_CANCELED) then
    let Some(tc) = duel::get_attacker(d) else { return };
    if card::can_attack(d, tc) && card::is_relate_to_battle(d, tc) && !card::is_status(d, tc, STATUS_ATTACK_CANCELED) {
        // local dam=tc:GetAttack()
        let dam = card::get_attack(d, tc) as i64;
        // if Duel.Damage(tp,dam,REASON_EFFECT)>0 and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_REMOVED) then
        if duel::damage(d, a.tp, dam, REASON_EFFECT, false, None) > 0
            && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
            && card::is_location(d, tc, LOCATION_REMOVED)
        {
            let h = effect::get_handler(d, a.e).nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetCondition(s.spcon)
            effect::set_condition(d, e1, Some(condition(spcon)));
            // e1:SetOperation(s.spop)
            effect::set_operation(d, e1, Some(operation(spop)));
            // e1:SetLabelObject(tc)
            effect::set_label_object(d, e1, Value::Card(tc));
            // e1:SetLabel(Duel.GetTurnCount())
            let turn = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[turn]);
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return Duel.GetTurnCount()~=e:GetLabel() and Duel.GetTurnPlayer()~=tp
    //     and tc:GetFlagEffect(id)~=0 and tc:GetReasonEffect():GetHandler()==e:GetHandler()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
        && duel::get_turn_player(d) != a.tp
        && card::get_flag_effect(d, tc, ID) != 0
        && {
            let re = card::get_reason_effect(d, tc).nil();
            let rh = effect::get_handler(d, re).nil();
            let h = effect::get_handler(d, a.e).nil();
            rh == h
        }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.SpecialSummon(e:GetLabelObject(),0,tp,tp,false,false,POS_FACEUP)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
