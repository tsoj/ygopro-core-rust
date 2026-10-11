//! Interdimensional Warp
use super::prelude::*;

pub const ID: u32 = 57384901;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_A as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
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
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToChangeControler()
    card::is_able_to_change_controler(d, c)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCounter(COUNTER_A)>0 and c:IsAbleToChangeControler()
    card::get_counter(d, c, COUNTER_A as u16) > 0 && card::is_able_to_change_controler(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter2), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&filter2), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil)
    let mut g1 = duel::select_target(d, a.tp, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g1), 2, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let Some(Value::Group(g)) = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).into_iter().next() else {
        crate::lua_error("attempt to index a nil value")
    };
    // local a=g:GetFirst()
    let first = group::get_first(d, &g).nil();
    // local b=g:GetNext()
    let order = group::iter(d, &g);
    // if a:IsRelateToEffect(e) and b:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, first, a.e) {
        let second = order.get(1).copied().nil();
        if card::is_relate_to_effect(d, second, a.e) {
            // Duel.SwapControl(a,b)
            duel::swap_control(d, first, second, 0, 0);
        }
    }
}
