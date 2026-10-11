//! Double Attack
use super::prelude::*;

pub const ID: u32 = 34187685;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv=c:GetOriginalLevel()
    let lv = card::get_original_level(d, c) as i64;
    // return lv>1 and c:IsMonster() and c:IsDiscardable() and c:IsAbleToGraveAsCost()
    //   and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,nil,lv)
    lv > 1
        && card::is_monster(d, c)
        && card::is_discardable(d, c, None, None)
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_target(d, Some(&|d, x| filter2(d, x, lv)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.filter2(c,lv)`
fn filter2(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(lv-1) and not c:IsHasEffect(EFFECT_EXTRA_ATTACK)
    if !card::is_faceup(d, c) {
        return false;
    }
    // a level never lies below a negative bound
    let below = lv - 1 >= 0 && card::is_level_below(d, c, (lv - 1) as u32);
    below && card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK, None).is_empty()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter2(chkc,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label(d, a.e);
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter2(d, chkc, label);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_HAND,0,1,nil,tp)
        return duel::is_existing_matching_card(d, Some(&|d, x| filter1(d, x, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_HAND,0,1,1,nil,tp)
    let cg = duel::select_matching_card(d, tp, Some(&|d, x| filter1(d, x, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(cg,REASON_DISCARD|REASON_COST)
    duel::send_to_grave(d, cg.clone(), REASON_DISCARD | REASON_COST, None, None);
    // local lv=cg:GetFirst():GetLevel()
    let first = group::get_first(d, &cg).nil();
    let lv = card::get_level(d, first) as i64;
    // e:SetLabel(lv)
    effect::set_label(d, a.e, &[lv]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil,lv)
    duel::select_target(d, tp, Some(&|d, x| filter2(d, x, lv)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
