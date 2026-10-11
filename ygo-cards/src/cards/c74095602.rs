//! Change of Hero - Reflector Ray
use super::prelude::*;

pub const ID: u32 = 74095602;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // return tc:IsPreviousControler(tp) and tc:IsType(TYPE_FUSION) and tc:IsSetCard(SET_ELEMENTAL_HERO)
    //   and tc:IsLocation(LOCATION_GRAVE) and tc:IsReason(REASON_BATTLE)
    card::is_previous_controler(d, tc, a.tp)
        && card::is_type(d, tc, TYPE_FUSION)
        && card::is_set_card(d, tc, &[SET_ELEMENTAL_HERO as u16])
        && card::is_location(d, tc, LOCATION_GRAVE)
        && card::is_reason(d, tc, REASON_BATTLE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc) as i64;
    // e:SetLabel(lv)
    effect::set_label(d, a.e, &[lv]);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(lv*300)
    duel::set_target_param(d, (lv * 300) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,lv*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (lv * 300) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
