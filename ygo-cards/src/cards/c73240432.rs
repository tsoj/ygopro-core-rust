//! Edge Imp Cotton Eater
use super::prelude::*;

pub const ID: u32 = 73240432;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FRIGHTFUR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    pendulum::add_procedure(d, c, None, None);
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // aux.TargetBoolFunction(Card.IsType,TYPE_FUSION)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_FUSION))));
    effect::set_value(d, e1, 300);
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsSetCard(SET_FRIGHTFUR) and c:IsType(TYPE_FUSION) and c:IsFusionSummoned()
    card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]) && card::is_type(d, c, TYPE_FUSION) && card::is_fusion_summoned(d, c)
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| cfilter(d, x, a.tp), 1, NO_EX)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
// function s.damfilter(c)
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FRIGHTFUR) and c:IsMonster()
    card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]) && card::is_monster(d, c)
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&damfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local val=Duel.GetMatchingGroupCount(s.damfilter,tp,LOCATION_GRAVE,0,nil)*200
    let val = duel::get_matching_group_count(d, Some(&damfilter), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 200;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(val)
    duel::set_target_param(d, val as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,val)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, val as i32);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local val=Duel.GetMatchingGroupCount(s.damfilter,tp,LOCATION_GRAVE,0,nil)*200
    let val = duel::get_matching_group_count(d, Some(&damfilter), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 200;
    // Duel.Damage(p,val,REASON_EFFECT)
    duel::damage(d, p, val, REASON_EFFECT, false, None);
}
