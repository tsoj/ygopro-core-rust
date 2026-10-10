//! Rigorous Reaver
use super::prelude::*;

pub const ID: u32 = 39180960;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // destroyed
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local g1=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_HAND,0,1,1,nil)
    let mut g1 = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DISCARD as u64);
    // local g2=Duel.SelectMatchingCard(1-tp,aux.TRUE,1-tp,LOCATION_HAND,0,1,1,nil)
    let g2 = duel::select_matching_card(d, 1 - a.tp, None, 1 - a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SendtoGrave(g1,REASON_DISCARD|REASON_EFFECT)
    duel::send_to_grave(d, g1, REASON_DISCARD | REASON_EFFECT, None, None);
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if c==tc then tc=Duel.GetAttackTarget() end
    if tc == Some(c) {
        tc = duel::get_attack_target(d);
    }
    // if not tc:IsRelateToBattle() then return end
    let tc = tc.nil();
    if !card::is_relate_to_battle(d, tc) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value(d, e1, -500);
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
