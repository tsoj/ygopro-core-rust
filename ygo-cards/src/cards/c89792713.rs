//! Nordic Relic Laevateinn
use super::prelude::*;

pub const ID: u32 = 89792713;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_END|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_END | TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge=Effect.CreateEffect(c)
        let ge = effect::create_effect(d, c);
        // ge:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, ge, EVENT_BATTLE_DESTROYING);
        // ge:SetOperation(s.checkop)
        effect::set_operation(d, ge, Some(operation(checkop)));
        // Duel.RegisterEffect(ge,0)
        duel::register_effect(d, ge, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsFaceup() and tc:IsRelateToBattle() then
        if card::is_faceup(d, tc) && card::is_relate_to_battle(d, tc) {
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetFlagEffect(id)~=0
    card::is_faceup(d, c) && card::get_flag_effect(d, c, ID) != 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc)
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        // return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(aux.FALSE)
        duel::set_chain_limit(d, aux_false());
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
