//! Red-Eyes Burn
use super::prelude::*;

pub const ID: u32 = 71782404;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsReason(REASON_EFFECT|REASON_BATTLE) and c:IsPreviousSetCard(SET_RED_EYES) and c:GetBaseAttack()>0
    //     and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED)
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
        && card::is_previous_set_card(d, c, &[SET_RED_EYES as u16])
        && card::get_base_attack(d, c) > 0
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    let eg = a.eg();
    // if chkc then return eg:IsContains(chkc) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return eg.contains(chkc) && cfilter(d, chkc, e, tp);
    }
    // if chk==0 then return eg:IsExists(s.cfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| cfilter(d, c, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=eg:FilterSelect(tp,s.cfilter,1,1,nil,e,tp)
    let g = group::filter_select(d, &eg, tp, &|d, c| cfilter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,PLAYER_ALL,g:GetFirst():GetBaseAttack())
    let first = group::get_first(d, &g).nil();
    let atk = card::get_base_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, atk);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Damage(1-tp,tc:GetBaseAttack(),REASON_EFFECT,true)
        let atk = card::get_base_attack(d, tc) as i64;
        duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, true, None);
        // Duel.Damage(tp,tc:GetBaseAttack(),REASON_EFFECT,true)
        let atk = card::get_base_attack(d, tc) as i64;
        duel::damage(d, a.tp, atk, REASON_EFFECT, true, None);
        // Duel.RDComplete()
        duel::rd_complete(d);
    }
}
