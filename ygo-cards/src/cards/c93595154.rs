//! Judgment of the Branded
use super::prelude::*;

pub const ID: u32 = 93595154;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy all of opponent's monsters, that has ATK >= than the targeted monster's ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set itself from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(8) and c:IsType(TYPE_FUSION)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,c:GetAttack())
    card::is_faceup(d, c)
        && card::is_level_above(d, c, 8)
        && card::is_type(d, c, TYPE_FUSION)
        && {
            let atk = card::get_attack(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| des_filter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
        }
}
/// s.desfilter(c,atk)
fn des_filter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()>=atk
    card::is_faceup(d, c) && card::get_attack(d, c) >= atk
}
/// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tg=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let tg = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local atk=tg:GetFirst():GetAttack()
    let tc = group::get_first(d, &tg).nil();
    let atk = card::get_attack(d, tc);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,atk)
    let g = duel::get_matching_group(d, Some(&|d, x| des_filter(d, x, atk)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    let tc = match tc {
        None => return,
        Some(tc) => tc,
    };
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,tc:GetAttack())
    let atk = card::get_attack(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, x| des_filter(d, x, atk)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsReason(REASON_COST) and re and re:IsActivated() and re:IsMonsterEffect()
    //     and re:GetHandler():IsCode(CARD_ALBAZ) then
    let mut ok = false;
    if card::is_reason(d, c, REASON_COST) {
        if let Some(re) = a.re {
            if effect::is_activated(d, re) && effect::is_monster_effect(d, re) {
                let rc = effect::get_handler(d, re).nil();
                ok = card::is_code(d, rc, &[CARD_ALBAZ]);
            }
        }
    }
    if ok {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_SET)
        effect::set_category(d, e1, CATEGORY_SET);
        // e1:SetType(EFFECT_TYPE_QUICK_O)
        effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
        // e1:SetCode(EVENT_FREE_CHAIN)
        effect::set_code(d, e1, EVENT_FREE_CHAIN);
        // e1:SetRange(LOCATION_GRAVE)
        effect::set_range(d, e1, LOCATION_GRAVE);
        // e1:SetCountLimit(1,id)
        effect::set_count_limit(d, e1, 1, ID, 0, 0);
        // e1:SetHintTiming(TIMING_END_PHASE)
        effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
        // e1:SetCondition(s.setcon)
        effect::set_condition(d, e1, Some(condition(setcon)));
        // e1:SetTarget(s.settg)
        effect::set_target(d, e1, Some(cost_target(settg)));
        // e1:SetOperation(s.setop)
        effect::set_operation(d, e1, Some(operation(setop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// s.setcon(e,tp,eg,ep,ev,re,r,rp)
fn setcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END)
    duel::is_phase(d, PHASE_END)
}
/// s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsSSetable() end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(c.into()), 1, 0, 0);
    true
}
/// s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, a.tp, c, None, true);
    }
}
