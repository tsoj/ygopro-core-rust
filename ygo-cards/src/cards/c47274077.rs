//! Neos Force
use super::prelude::*;

pub const ID: u32 = 47274077;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NEOS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,CARD_NEOS))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_code(d, c, &[CARD_NEOS]))), None, None, None, None, None, None);
    // atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 800);
    card::register_effect(d, c, e2, false);
    // damage
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_condition(d, e4, Some(condition(damcon)));
    effect::set_target(d, e4, Some(cost_target(damtg)));
    effect::set_operation(d, e4, Some(operation(damop)));
    card::register_effect(d, c, e4, false);
    // return
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_TODECK);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_target(d, e5, Some(cost_target(rettg)));
    effect::set_operation(d, e5, Some(operation(retop)));
    card::register_effect(d, c, e5, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let g = a.eg.clone().nil();
    let ec = group::get_first(d, &g);
    // local bc=ec:GetBattleTarget()
    let ec = ec.nil();
    let bc = card::get_battle_target(d, ec);
    // return ec==e:GetHandler():GetEquipTarget() and bc:IsLocation(LOCATION_GRAVE) and bc:IsReason(REASON_BATTLE)
    let h = effect::get_handler(d, a.e).nil();
    if Some(ec) != card::get_equip_target(d, h) {
        return false;
    }
    let bc = bc.nil();
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_reason(d, bc, REASON_BATTLE)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ec=eg:GetFirst()
    let g = a.eg.clone().nil();
    let ec = group::get_first(d, &g);
    // local bc=ec:GetBattleTarget()
    let ec = ec.nil();
    let bc = card::get_battle_target(d, ec).nil();
    // local dam=bc:GetAttack()
    let dam = card::get_attack(d, bc);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int() as i64, REASON_EFFECT, false, None);
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, h.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.SendtoDeck(e:GetHandler(),nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_deck(d, h, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
