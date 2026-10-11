//! Sargasso the D.D. Battlefield
use super::prelude::*;

pub const ID: u32 = 1127737;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.damcon1)
    effect::set_condition(d, e2, Some(condition(damcon1)));
    // e2:SetTarget(s.damtg1)
    effect::set_target(d, e2, Some(cost_target(damtg1)));
    // e2:SetOperation(s.damop1)
    effect::set_operation(d, e2, Some(operation(damop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e2=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2b, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2b, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2b, 1, 0, 0, 0);
    // e2:SetCondition(s.damcon2)
    effect::set_condition(d, e2b, Some(condition(damcon2)));
    // e2:SetOperation(s.damop2)
    effect::set_operation(d, e2b, Some(operation(damop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2b, false);
}
/// `s.damcon1(e,tp,eg,ep,ev,re,r,rp)`
fn damcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsXyzSummoned()
    let eg = a.eg.clone().nil();
    let f = group::get_first(d, &eg).nil();
    card::is_xyz_summoned(d, f)
}
/// `s.damtg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    let f = group::get_first(d, &eg).nil();
    // Duel.SetTargetPlayer(eg:GetFirst():GetSummonPlayer())
    let sp = card::get_summon_player(d, f);
    duel::set_target_player(d, sp);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,eg:GetFirst():GetSummonPlayer(),500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, sp, 500);
    true
}
/// `s.damop1(e,tp,eg,ep,ev,re,r,rp)`
fn damop1(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        let p = info[0].int() as u8;
        let amount = info[1].int();
        // if not Duel.IsPlayerAffectedByEffect(p,37511832) then
        if duel::is_player_affected_by_effect(d, p, 37511832).is_none() {
            // Duel.Damage(p,d,REASON_EFFECT)
            duel::damage(d, p, amount, REASON_EFFECT, false, None);
        }
    }
}
/// `s.damcon2(e,tp,eg,ep,ev,re,r,rp)`
fn damcon2(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_XYZ),Duel.GetTurnPlayer(),LOCATION_MZONE,0,1,nil)
    let tp = duel::get_turn_player(d);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.damop2(e,tp,eg,ep,ev,re,r,rp)`
fn damop2(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // if not Duel.IsPlayerAffectedByEffect(p,37511832) then
    if duel::is_player_affected_by_effect(d, p, 37511832).is_none() {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Damage(p,500,REASON_EFFECT)
        duel::damage(d, p, 500, REASON_EFFECT, false, None);
    }
}
