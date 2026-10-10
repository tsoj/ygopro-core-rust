//! Cardcar D
use super::prelude::*;

pub const ID: u32 = 45812361;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e2, Some(operation(sumsuc)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DRAW);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(cond)));
    effect::set_cost(d, e3, Some(cost_target(cost)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
/// `s.sumsuc(e,...)`
fn sumsuc(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
}
/// `s.condition(e,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0 and Duel.IsPhase(PHASE_MAIN1)
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0 && duel::is_phase(d, PHASE_MAIN1)
}
/// `s.cost(e,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 and e:GetHandler():IsReleasable() end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] == 0 && card::is_releasable(d, h, None, None);
    }
    // Duel.Release(e:GetHandler(),REASON_COST)
    duel::release(d, h, REASON_COST, None);
    // oath effects
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.target(e,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.operation(e,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.Draw(tp,2,REASON_EFFECT)
    let ct = duel::draw(d, tp, 2, REASON_EFFECT);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SkipPhase(tp,PHASE_MAIN1,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, tp, PHASE_MAIN1, RESET_PHASE | PHASE_END, 1, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
