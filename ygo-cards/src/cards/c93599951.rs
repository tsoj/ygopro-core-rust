//! Dark Spirit of the Silent
use super::prelude::*;

pub const ID: u32 = 93599951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    if a.chk == Some(0) {
        // return a and a:IsCanBeEffectTarget(e) and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,a)
        let Some(atk) = atk else { return false };
        return card::is_can_be_effect_target(d, atk, Some(a.e))
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, atk.into());
    }
    let atk = atk.nil();
    // Duel.SetTargetCard(a)
    duel::set_target_card(d, atk);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,a)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, atk.into()).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = match group::get_first(d, &g) {
        Some(tc) => Value::Card(tc),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, first);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if Duel.NegateAttack() and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if duel::negate_attack(d) && card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // if tc:IsDefensePos() then Duel.ChangePosition(tc,POS_FACEUP_ATTACK) end
        if card::is_defense_pos(d, tc) {
            duel::change_position(d, tc, POS_FACEUP_ATTACK as u8, None, None, None, false);
        }
        // if tc:CanAttack() and not tc:IsImmuneToEffect(e) then
        if card::can_attack(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.CalculateDamage(tc,Duel.GetAttackTarget())
            let at = duel::get_attack_target(d);
            duel::calculate_damage(d, tc, at, false);
        }
    }
}
