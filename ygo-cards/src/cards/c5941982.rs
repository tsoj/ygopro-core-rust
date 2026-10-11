//! Scareclaw Alternative
use super::prelude::*;

pub const ID: u32 = 5941982;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SCARECLAW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,3,false,nil,nil,SET_SCARECLAW) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_SCARECLAW as u16])), 3, 3, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,3,3,false,nil,nil,SET_SCARECLAW)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_SCARECLAW as u16])), 3, 3, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    let _ = duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Can only Special Summon in Defense Position
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_FORCE_SPSUMMON_POSITION)
    effect::set_code(d, e1, EFFECT_FORCE_SPSUMMON_POSITION);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(POS_DEFENSE)
    effect::set_value(d, e1, POS_DEFENSE as i64);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `local dg=Duel.GetMatchingGroup(Card.IsDefensePos,0,LOCATION_MZONE,LOCATION_MZONE,nil)`
fn defense_group(d: &mut Duel) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_defense_pos(d, c)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let dg = defense_group(d);
    // if chk==0 then return #dg>0 end
    if a.chk == Some(0) {
        return !dg.is_empty();
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#dg*100)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (dg.len() * 100) as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    let dg = defense_group(d);
    // if #dg==0 then return end
    if dg.is_empty() {
        return;
    }
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    // Duel.Damage(p,#dg*100,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, (dg.len() * 100) as i64, REASON_EFFECT, false, None);
}
