//! Sakuretsu Armor
use super::prelude::*;

pub const ID: u32 = 56120475;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
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
    let tg = tg.nil();
    duel::set_target_card(d, tg);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tg)), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:CanAttack() and not tc:IsStatus(STATUS_ATTACK_CANCELED) then
    if card::is_relate_to_effect(d, tc, a.e) && card::can_attack(d, tc) && !card::is_status(d, tc, STATUS_ATTACK_CANCELED) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
