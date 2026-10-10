//! Evilswarm Exciton Knight
use super::prelude::*;

pub const ID: u32 = 46772449;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Destroy all other cards on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(TIMING_SPSUMMON,TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, TIMING_SPSUMMON, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if not ((Duel.IsTurnPlayer(tp) and Duel.IsMainPhase()) or (Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase())) then return false end
    if !((duel::is_turn_player(d, tp) && duel::is_main_phase(d, None)) || (duel::is_turn_player(d, 1 - tp) && duel::is_battle_phase(d, None))) {
        return false;
    }
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND|LOCATION_ONFIELD)>Duel.GetFieldGroupCount(tp,LOCATION_HAND|LOCATION_ONFIELD,0)
    duel::get_field_group_count(d, tp, 0, LOCATION_HAND | LOCATION_ONFIELD) > duel::get_field_group_count(d, tp, LOCATION_HAND | LOCATION_ONFIELD, 0)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,exc)
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    // --Your opponent takes no damage for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
