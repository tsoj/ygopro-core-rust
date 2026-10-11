//! Extox Hydra
use super::prelude::*;

pub const ID: u32 = 26050548;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixRep(c,true,true,s.ffilter,2,99)
    fusion::add_proc_mix_rep(d, c, true, true, fusion::Mat::F(fusion::fus_mat(ffilter)), 2, 99, vec![]);
    // Check materials used for Fusion Summon
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e0, effect_card_value(matcheck));
    card::register_effect(d, c, e0, false);
    // Reduce ATK
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    effect::set_target(d, e1, Some(effect_card_fn(e1_target)));
    effect::set_value_fn(d, e1, effect_card_value(e1_value));
    card::register_effect(d, c, e1, false);
    // Draw cards
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, _fc: CardId, _sumtype: u64, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let mats = card::get_material(d, c);
    let bits = group::get_bitwise_or(d, &mats, &|d, x| card::get_type(d, x) as i64);
    let typ = bits & (TYPE_EXTRA | TYPE_PENDULUM) as u64;
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 0, 1, typ as i64, 0);
    0
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `function(e,c) return c:IsType(e:GetHandler():GetFlagEffectLabel(id)) end`
fn e1_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let typ = card::get_flag_effect_label(d, h, ID).first().copied().unwrap_or(0);
    card::is_type(d, c, typ as u32)
}
/// `function(_,c) return -c:GetBaseAttack() end`
fn e1_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    -(card::get_base_attack(d, c) as i64)
}
/// `function(_,tp,_,ep,ev) return ep==1-tp and ev>=1000 end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let _ = d;
    a.ep == 1 - a.tp && a.ev >= 1000
}
/// `s.drtg(e,tp,...,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ct = a.ev / 1000;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, ct);
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, ct);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    true
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
