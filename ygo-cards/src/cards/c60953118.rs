//! Arcana Force XIV - Temperance
use super::prelude::*;

pub const ID: u32 = 60953118;
pub const INFO: ScriptInfo = ScriptInfo { arcanareg: Some(arcanareg), toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Discard to prevent battle damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_condition(d, e1, Some(condition(damcon)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
    // Toss a coin and apply the appropriate effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_COIN);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(cointg)));
    effect::set_operation(d, e2, Some(operation(coinop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e2);
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetBattleDamage(tp)>0
    duel::get_battle_damage(d, a.tp) > 0
}
fn damop(d: &mut Duel, a: &EffArgs) {
    // Prevent Battle Damage
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // s.arcanareg(c,Arcana.TossCoin(c,tp))
    let coin = aux::arcana::toss_coin(d, c, a.tp);
    arcanareg(d, c, coin);
}
fn arcanareg(d: &mut Duel, c: CardId, coin: i32) {
    // Heads: Halve all Battle Damage you take
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(rdcon1)));
    effect::set_value(d, e1, HALF_DAMAGE as i64);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    // Tails: Halve all Battle Damage your opponent take
    let e2 = effect::clone(d, e1);
    effect::set_target_range(d, e2, 0, 1);
    effect::set_condition(d, e2, Some(condition(rdcon2)));
    card::register_effect(d, c, e2, false);
    aux::arcana::register_coin_result(d, c, coin);
}
fn rdcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Arcana.GetCoinResult(e:GetHandler())==COIN_HEADS
    let h = effect::get_handler(d, a.e).nil();
    aux::arcana::get_coin_result(d, h) == Some(COIN_HEADS as i64)
}
fn rdcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Arcana.GetCoinResult(e:GetHandler())==COIN_TAILS
    let h = effect::get_handler(d, a.e).nil();
    aux::arcana::get_coin_result(d, h) == Some(COIN_TAILS as i64)
}
