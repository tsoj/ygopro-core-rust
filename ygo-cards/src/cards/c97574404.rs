//! Arcana Force VI - The Lovers
use super::prelude::*;

pub const ID: u32 = 97574404;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCANA_FORCE], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // coin
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COIN);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(cointg)));
    effect::set_operation(d, e1, Some(operation(coinop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
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
    // coin effect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DOUBLE_TRIBUTE);
    effect::set_condition(d, e1, Some(condition(dtcon)));
    effect::set_value_fn(d, e1, lua_fn(dt_value));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_condition(d, e2, Some(condition(sumcon)));
    effect::set_target(d, e2, Some(lua_fn(sumtg)));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_CANNOT_MSET);
    card::register_effect(d, c, e3, false);
    aux::arcana::register_coin_result(d, c, coin);
}
fn dtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Arcana.GetCoinResult(e:GetHandler())==COIN_HEADS
    let h = effect::get_handler(d, a.e).nil();
    aux::arcana::get_coin_result(d, h) == Some(COIN_HEADS as i64)
}
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Arcana.GetCoinResult(e:GetHandler())==COIN_TAILS
    let h = effect::get_handler(d, a.e).nil();
    aux::arcana::get_coin_result(d, h) == Some(COIN_TAILS as i64)
}
/// `function(_,c) return c:IsSetCard(SET_ARCANA_FORCE) end`
fn dt_value(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16])
}
/// `s.sumtg(e,c,tp,sumtp)`: `return (sumtp&SUMMON_TYPE_TRIBUTE)==SUMMON_TYPE_TRIBUTE and c:IsSetCard(SET_ARCANA_FORCE)`
fn sumtg(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtp = a.int(3);
    (sumtp & SUMMON_TYPE_TRIBUTE as i64) == SUMMON_TYPE_TRIBUTE as i64 && card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16])
}
