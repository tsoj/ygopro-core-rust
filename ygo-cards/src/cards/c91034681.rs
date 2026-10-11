//! Frightfur Daredevil
//! デストーイ・デアデビル
use super::prelude::*;

pub const ID: u32 = 91034681;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FRIGHTFUR, SET_EDGE_IMP, SET_FLUFFAL],
    material_setcode: &[SET_EDGE_IMP, SET_FLUFFAL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_EDGE_IMP),aux.FilterBoolFunctionEx(Card.IsSetCard,SET_FLUFFAL))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_EDGE_IMP as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_FLUFFAL as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // damage after destroying
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCondition(aux.bdocon)
    effect::set_condition(d, e1, Some(aux::bdocon()));
    // e1:SetTarget(s.damtg1)
    effect::set_target(d, e1, Some(cost_target(damtg1)));
    // e1:SetOperation(s.damop1)
    effect::set_operation(d, e1, Some(operation(damop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage after destruction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.damcon2)
    effect::set_condition(d, e2, Some(condition(damcon2)));
    // e2:SetTarget(s.damtg2)
    effect::set_target(d, e2, Some(cost_target(damtg2)));
    // e2:SetOperation(s.damop2)
    effect::set_operation(d, e2, Some(operation(damop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.damtg1(e,tp,eg,ep,ev,re,r,rp,chk)`
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
/// `s.damop1(e,tp,eg,ep,ev,re,r,rp)`
fn damop1(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.damcon2(e,tp,eg,ep,ev,re,r,rp)`
fn damcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (c:IsReason(REASON_BATTLE) or (c:GetReasonPlayer()~=tp and c:GetOwner()==c:GetPreviousControler() and c:IsReason(REASON_EFFECT)))
    //     and c:IsPreviousPosition(POS_FACEUP)
    (card::is_reason(d, c, REASON_BATTLE)
        || (card::get_reason_player(d, c) != a.tp && card::get_owner(d, c) == card::get_previous_controler(d, c) && card::is_reason(d, c, REASON_EFFECT)))
        && card::is_previous_position(d, c, POS_FACEUP as u32)
}
/// `s.damfilter(c)`
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_FRIGHTFUR)
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_FRIGHTFUR as u16])
}
/// `s.damtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // local gc=Duel.GetMatchingGroupCount(s.damfilter,tp,LOCATION_GRAVE,0,nil)
    let gc = duel::get_matching_group_count(d, Some(&damfilter), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // if chk==0 then return gc>0 end
    if a.chk == Some(0) {
        return gc > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,gc*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (gc * 500) as i32);
    true
}
/// `s.damop2(e,tp,eg,ep,ev,re,r,rp)`
fn damop2(d: &mut Duel, a: &EffArgs) {
    // local gc=Duel.GetMatchingGroupCount(s.damfilter,tp,LOCATION_GRAVE,0,nil)
    let gc = duel::get_matching_group_count(d, Some(&damfilter), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Damage(p,gc*500,REASON_EFFECT)
    duel::damage(d, p, gc * 500, REASON_EFFECT, false, None);
}
