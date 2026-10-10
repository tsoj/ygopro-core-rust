//! Joruri-P.U.N.K. Dangerous Gabu
use super::prelude::*;

pub const ID: u32 = 43685562;

// s.listed_series={SET_PUNK}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the effects of 1 Effect Monster your opponent controls, then, if you control a "P.U.N.K." monster, gain LP equal to that targeted monster's original ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsNegatableMonster() and chkc:IsEffectMonster() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_negatable_monster(d, chkc)
            && card::is_effect_monster(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsNegatableMonster,Card.IsEffectMonster),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable_monster(d, c) && card::is_effect_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local tc=Duel.SelectTarget(tp,aux.AND(Card.IsNegatableMonster,Card.IsEffectMonster),tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable_monster(d, c) && card::is_effect_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    // (g is not a local of this script: the Lua global is nil)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, a.tp, 0);
    // local atk=tc:GetBaseAttack()
    let atk = card::get_base_attack(d, tc);
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PUNK),tp,LOCATION_MZONE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PUNK as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,atk)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, atk);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,atk)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, atk);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsCanBeDisabledByEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_can_be_disabled_by_effect(d, tc, a.e, true) {
        // tc:NegateEffects(e:GetHandler(),RESETS_STANDARD_PHASE_END)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESETS_STANDARD_PHASE_END), false, None);
        // if not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PUNK),tp,LOCATION_MZONE,0,1,nil) then return end
        if !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PUNK as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
            return;
        }
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // local atk=tc:GetBaseAttack()
        let atk = card::get_base_attack(d, tc);
        // if tc:IsDisabled() and atk>0 then
        if card::is_disabled(d, tc) && atk > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Recover(tp,atk,REASON_EFFECT)
            duel::recover(d, a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
