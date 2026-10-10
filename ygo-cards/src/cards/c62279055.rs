//! Magic Cylinder
use super::prelude::*;

pub const ID: u32 = 62279055;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==1-Duel.GetTurnPlayer()
    a.tp == 1 - duel::get_turn_player(d)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return tg and tg:IsOnField() and tg:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        return match tg {
            Some(tg) => card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e)),
            None => false,
        };
    }
    // Duel.SetTargetCard(tg)
    let tg = tg.nil();
    duel::set_target_card(d, tg);
    // local dam=tg:GetAttack()
    let dam = card::get_attack(d, tg);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // if Duel.NegateAttack() then
            if duel::negate_attack(d) {
                // Duel.Damage(1-tp,tc:GetAttack(),REASON_EFFECT)
                let atk = card::get_attack(d, tc);
                duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
            }
        }
    }
}
