//! Fairy Box
use super::prelude::*;

pub const ID: u32 = 21598948;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    card::register_effect(d, c, e1, false);
    // Change the ATK of a monster to 0
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_COIN);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_target(d, e2, Some(cost_target(atktg2)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
    // Maintenance cost
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(turn_con)));
    effect::set_operation(d, e3, Some(operation(mtop)));
    card::register_effect(d, c, e3, false);
}
fn turn_con(d: &mut Duel, a: &EffArgs) -> bool {
    // function(_,tp) return Duel.IsTurnPlayer(tp) end
    duel::is_turn_player(d, a.tp)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    // return at and at:IsControler(1-tp)
    match duel::get_attacker(d) {
        Some(at) => card::is_controler(d, at, 1 - a.tp),
        None => false,
    }
}
fn atktg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(Duel.GetAttacker())
    let at = duel::get_attacker(d);
    duel::set_target_card(d, at.nil());
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // if not a:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, atk, a.e) {
        return;
    }
    // if Duel.CallCoin(tp,1) then
    if duel::call_coin(d, a.tp) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
        // a:RegisterEffect(e1)
        card::register_effect(d, atk, e1, false);
    }
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckLPCost(tp,500) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::check_lp_cost(d, a.tp, 500) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.PayLPCost(tp,500)
        duel::pay_lp_cost(d, a.tp, 500);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
