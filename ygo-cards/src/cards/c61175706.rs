//! Arcana Force IV - The Emperor
use super::prelude::*;

pub const ID: u32 = 61175706;
pub const INFO: ScriptInfo = ScriptInfo { arcanareg: Some(arcanareg), listed_series: &[SET_ARCANA_FORCE], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin and apply the appropriate effect
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
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16]))));
    effect::set_value_fn(d, e1, effect_value(atkval));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    //
    aux::arcana::register_coin_result(d, c, coin);
}
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local coin=Arcana.GetCoinResult(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let coin = aux::arcana::get_coin_result(d, h);
    // if coin==COIN_HEADS then return 500 elseif coin==COIN_TAILS then return -500 else return 0 end
    if coin == Some(COIN_HEADS as i64) {
        500
    } else if coin == Some(COIN_TAILS as i64) {
        -500
    } else {
        0
    }
}
