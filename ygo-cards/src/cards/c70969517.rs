//! Phantom Beast Rock-Lizard
use super::prelude::*;

pub const ID: u32 = 70969517;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PHANTOM_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    // damage1
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_target(d, e2, Some(cost_target(damtg1)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
    // damage2
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(damcon2)));
    effect::set_target(d, e2, Some(cost_target(damtg2)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_PHANTOM_BEAST) and (c:IsControler(tp) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_PHANTOM_BEAST as u16]) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
fn damtg1(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 500);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
fn damcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&(REASON_DESTROY|REASON_EFFECT))==(REASON_DESTROY|REASON_EFFECT) and rp~=tp and e:GetHandler():IsPreviousControler(tp)
    (a.r & (REASON_DESTROY | REASON_EFFECT)) == (REASON_DESTROY | REASON_EFFECT)
        && a.rp != a.tp
        && card::is_previous_controler(d, effect::get_handler(d, a.e).nil(), a.tp)
}
fn damtg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 2000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 2000);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
