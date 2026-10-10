//! Mechquipped Angineer
//! 機装天使エンジネル
use super::prelude::*;

pub const ID: u32 = 15914410;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz summon procedure
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // Targeted monster cannot be destroyed by battle or card effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`: `c:IsPosition(POS_FACEUP_ATTACK) and c:IsCanChangePosition()`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_can_change_position(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUPATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUPATTACK as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)
    duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    // Cannot be destroyed by battle or card effect
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(3008)
    effect::set_description(d, e1, 3008);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
