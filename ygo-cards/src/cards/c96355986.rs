//! Enchanted Javelin
use super::prelude::*;

pub const ID: u32 = 96355986;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `function s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
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
    let tg = tg.nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    // local rec=tg:GetAttack()
    let rec = card::get_attack(d, tg);
    // Duel.SetTargetParam(rec)
    duel::set_target_param(d, rec as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, rec);
    true
}
/// `function s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local rec=tc:GetAttack()
        let rec = card::get_attack(d, tc);
        // Duel.Recover(tp,rec,REASON_EFFECT)
        duel::recover(d, a.tp, rec as i64, REASON_EFFECT, false, None);
    }
}
