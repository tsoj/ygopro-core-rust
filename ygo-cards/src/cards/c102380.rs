//! Lava Golem
use super::prelude::*;

pub const ID: u32 = 102380;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // aux.AddLavaProcedure(c,2,POS_FACEUP,nil,0,aux.Stringid(id,0))
    aux::add_lava_procedure(d, c, Some(2), POS_FACEUP, None, Some(0), Some(aux::stringid(ID, 0)));
    // Take 1000 damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
    // Check/apply Normal Summon/Set restriction
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EFFECT_SPSUMMON_COST);
    // e3:SetCost(function(_,_,tp) return Duel.GetActivityCount(tp,ACTIVITY_NORMALSUMMON)==0 end)
    effect::set_cost(d, e3, Some(lua_fn(|d, a| duel::get_activity_count(d, a.p(2), &[ACTIVITY_NORMALSUMMON]).first() == Some(&0))));
    effect::set_operation(d, e3, Some(operation(spcostop)));
    card::register_effect(d, c, e3, false);
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 1000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.spcostop(e,tp,eg,ep,ev,re,r,rp)`
fn spcostop(d: &mut Duel, a: &EffArgs) {
    // Cannot Normal Summon/Set
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_MSET);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
