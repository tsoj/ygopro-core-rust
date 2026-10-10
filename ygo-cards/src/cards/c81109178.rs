//! Fire Cracker
use super::prelude::*;

pub const ID: u32 = 81109178;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0x42, None, None);
    // damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(damtg1)));
    effect::set_operation(d, e1, Some(operation(damop1)));
    card::register_effect(d, c, e1, false);
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_DAMAGE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(ctcon)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
    // damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(damtg2)));
    effect::set_operation(d, e3, Some(operation(damop2)));
    card::register_effect(d, c, e3, false);
}
fn damtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
fn damop1(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dmg = info[1].int();
    // if Duel.Damage(p,d,REASON_EFFECT)~=0 then
    if duel::damage(d, p, dmg, REASON_EFFECT, false, None) != 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_SKIP_DP)
        effect::set_code(d, e1, EFFECT_SKIP_DP);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_DRAW) then
        if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_DRAW) {
            // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 2);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
fn ctcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and (r&REASON_EFFECT)~=0
    a.ep != a.tp && (a.r & REASON_EFFECT) != 0
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x42,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x42, 1, false);
}
fn damtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=e:GetHandler():GetCounter(0x42)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, 0x42);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(ct)
    duel::set_target_param(d, ct as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, ct);
    true
}
fn damop2(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(0x42)
    let ct = card::get_counter(d, c, 0x42);
    // if c:RemoveCounter(tp,0x42,ct,REASON_EFFECT) then
    if card::remove_counter(d, c, a.tp, 0x42, ct as u16, REASON_EFFECT) {
        // Duel.Damage(p,ct*300,REASON_EFFECT)
        duel::damage(d, p, ct as i64 * 300, REASON_EFFECT, false, None);
    }
}
