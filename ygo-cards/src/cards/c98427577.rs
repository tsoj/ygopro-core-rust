//! Scrap-Iron Scarecrow
use super::prelude::*;

pub const ID: u32 = 98427577;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.target(e,...,chk,chkc)`
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
    duel::set_target_card(d, tg.nil());
    true
}
/// `s.activate(e,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateAttack()
    duel::negate_attack(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if c:IsSSetable(true) and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if card::is_s_setable(d, c, true, None) && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.ChangePosition(c,POS_FACEDOWN)
        duel::change_position(d, c, POS_FACEDOWN as u8, None, None, None, false);
        // Duel.RaiseEvent(c,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, c, EVENT_SSET, Some(a.e), REASON_EFFECT, a.tp, a.tp, 0);
    }
}
