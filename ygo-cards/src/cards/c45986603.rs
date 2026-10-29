//! Snatch Steal
use super::prelude::*;

pub const ID: u32 = 45986603;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,1,aux.CheckStealEquip,s.eqlimit,nil,s.target)
    aux::add_equip_procedure(
        d,
        c,
        Some(1),
        Some(aux::equip_filter_fn(|d, c, e, tp| aux::check_steal_equip(d, c, e, tp))),
        Some(effect_card_fn(eqlimit)),
        None,
        Some(std::sync::Arc::new(target)),
        None,
        None,
        None,
    );
    // recover
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e2, CATEGORY_RECOVER);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(reccon)));
    effect::set_target(d, e2, Some(cost_target(rectg)));
    effect::set_operation(d, e2, Some(operation(recop)));
    card::register_effect(d, c, e2, false);
    // control
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_SET_CONTROL);
    effect::set_value_fn(d, e4, effect_value(ctval));
    card::register_effect(d, c, e4, false);
}
/// `s.eqlimit(e,c)`: `e:GetHandlerPlayer()~=c:GetControler() or e:GetHandler():GetEquipTarget()==c`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_handler_player(d, e) != card::get_controler(d, c) || card::get_equip_target(d, effect::get_handler(d, e).nil()) == Some(c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,tc)`
fn target(d: &mut Duel, a: &EffArgs, tc: Option<CardId>) {
    // e:SetCategory(CATEGORY_CONTROL+CATEGORY_EQUIP)
    effect::set_category(d, a.e, CATEGORY_CONTROL + CATEGORY_EQUIP);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, tc.map(CardSet::from_card), 1, 0, 0);
}
/// `s.reccon(e,tp,eg,ep,ev,re,r,rp)`
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - a.tp, 1000);
    true
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.ctval(e,c)`: `return e:GetHandlerPlayer()`
fn ctval(d: &mut Duel, e: EffectId) -> i64 {
    effect::get_handler_player(d, e) as i64
}
