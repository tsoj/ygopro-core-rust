//! Poseidon Wave
use super::prelude::*;

pub const ID: u32 = 25642998;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FISH|RACE_SEASERPENT|RACE_AQUA)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return tg:IsOnField() and tg:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e));
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.nil());
    // local dam=Duel.GetMatchingGroupCount(s.dfilter,tp,LOCATION_MZONE,0,nil)*800
    let dam = duel::get_matching_group_count(d, Some(&dfilter), tp, LOCATION_MZONE, 0, NO_EX) as i64 * 800;
    // if dam>0 then
    if dam > 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.NegateAttack() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::negate_attack(d) {
            // local dam=Duel.GetMatchingGroupCount(s.dfilter,tp,LOCATION_MZONE,0,nil)*800
            let dam = duel::get_matching_group_count(d, Some(&dfilter), tp, LOCATION_MZONE, 0, NO_EX) as i64 * 800;
            // if dam>0 then
            if dam > 0 {
                // Duel.Damage(1-tp,dam,REASON_EFFECT)
                duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
            }
        }
    }
}
