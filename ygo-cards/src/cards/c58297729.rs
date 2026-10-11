//! Transmission Gear
use super::prelude::*;

pub const ID: u32 = 58297729;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if chk==0 then
    if a.chk == Some(0) {
        // return d and a:GetControler()~=d:GetControler()
        //     and a:IsAbleToRemove(a:GetControler(),POS_FACEDOWN,REASON_RULE)
        //     and d:IsAbleToRemove(d:GetControler(),POS_FACEDOWN,REASON_RULE)
        let Some(dt) = dt else {
            return false;
        };
        let atk = atk.nil();
        let ac = card::get_controler(d, atk);
        let dc = card::get_controler(d, dt);
        return ac != dc
            && card::is_able_to_remove(d, atk, Some(ac), Some(POS_FACEDOWN as u8), Some(REASON_RULE))
            && card::is_able_to_remove(d, dt, Some(dc), Some(POS_FACEDOWN as u8), Some(REASON_RULE));
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let mut atk = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let mut dt = duel::get_attack_target(d).nil();
    // if not a:IsRelateToBattle() or not d:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, atk) {
        return;
    }
    if !card::is_relate_to_battle(d, dt) {
        return;
    }
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, atk, 1 - a.tp) {
        std::mem::swap(&mut atk, &mut dt);
    }
    // local res=Duel.RockPaperScissors()
    let res = duel::rock_paper_scissors(d, true);
    // if res==tp then
    if res == a.tp as i32 {
        // Duel.Remove(d,POS_FACEDOWN,REASON_RULE)
        duel::remove(d, dt, POS_FACEDOWN as u8, REASON_RULE, None, None);
    } else {
        // Duel.Remove(a,POS_FACEDOWN,REASON_RULE)
        duel::remove(d, atk, POS_FACEDOWN as u8, REASON_RULE, None, None);
    }
}
