//! Trickstar Divaridis
use super::prelude::*;

pub const ID: u32 = 14365823;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link summon
    // Link.AddProcedure(c,s.matfilter,2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, lc, sumtype, tp| matfilter(d, c, lc, sumtype, tp))), 2, Some(2), None, None, None);
    // Must be properly summoned in order to be revived
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Can only control one
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Effect damage, optional trigger
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetTarget(s.sstg)
    effect::set_target(d, e1, Some(cost_target(sstg)));
    // e1:SetOperation(s.ssop)
    effect::set_operation(d, e1, Some(operation(ssop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Effect damage for normal summon, continuous effect
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Effect damage for special summon, continuous effect
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelBelow(3) and c:IsSetCard(SET_TRICKSTAR,lc,sumtype,tp)
    card::is_level_below(d, c, 3) && card::is_set_card_ctx(d, c, &[SET_TRICKSTAR as u16], lc, sumtype, Some(tp))
}
/// `s.sstg(e,tp,eg,ep,ev,re,r,rp,chk)`: "Activation legality"
fn sstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(200)
    duel::set_target_param(d, 200);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 200);
    true
}
/// `s.ssop(e,tp,eg,ep,ev,re,r,rp)`: "Performing the effect damage"
fn ssop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int() as i64, REASON_EFFECT, false, None);
}
/// `s.filter(c,p)`: "If opponent was the one to perform the summon"
fn filter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return c:GetSummonPlayer()==p
    card::get_summon_player(d, c) == p
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`: "If it ever happened"
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - a.tp), 1, NO_EX)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`: "Inflict effect damage to opponent"
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(200)
    duel::set_target_param(d, 200);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 200);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int() as i64, REASON_EFFECT, false, None);
}
