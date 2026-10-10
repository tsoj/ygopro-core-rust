//! Dark Sanctuary
use super::prelude::*;

pub const ID: u32 = 16625614;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DESTINY_BOARD], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // "Spirit Message" handling
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_code(d, e2, ID);
    effect::set_range(d, e2, LOCATION_FZONE);
    card::register_effect(d, c, e2, false);
    // Toss a coin, negate attack and inflict damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e3, CATEGORY_COIN);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_condition(d, e3, Some(condition(cond)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    // return at and at:IsControler(1-tp)
    match duel::get_attacker(d) {
        Some(at) => card::is_controler(d, at, 1 - a.tp),
        None => false,
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // local coin=Duel.TossCoin(tp,1)
    let coin = duel::toss_coin(d, a.tp, 1)[0];
    // if coin==COIN_HEADS and Duel.NegateAttack() then
    if coin == COIN_HEADS as i32 && duel::negate_attack(d) {
        // Duel.Damage(1-tp,math.ceil(tc:GetAttack()/2),REASON_EFFECT)
        let atk = card::get_attack(d, tc.nil());
        let dmg = (atk as f64 / 2.0).ceil() as i64;
        duel::damage(d, 1 - a.tp, dmg, REASON_EFFECT, false, None);
    }
}
