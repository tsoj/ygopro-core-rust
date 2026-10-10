//! Invader of the Throne
use super::prelude::*;

pub const ID: u32 = 3056267;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.swapfilter(c)`
fn swapfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToChangeControler()
    card::is_able_to_change_controler(d, c)
}
/// `s.condition(e)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return not Duel.IsBattlePhase()
    !duel::is_battle_phase(d, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(s.swapfilter,tp,0,LOCATION_MZONE,1,nil) then
    if duel::is_existing_target(d, Some(&|d, c| swapfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local mon=Duel.SelectTarget(tp,s.swapfilter,tp,0,LOCATION_MZONE,1,1,nil)
        let mut mon = duel::select_target(d, a.tp, Some(&|d, c| swapfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // mon:AddCard(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        group::add_card(&mut mon, &CardSet::from_card(h));
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,mon,2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(mon), 2, 0, 0);
    }
    false
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and tc and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                // Duel.SwapControl(c,tc)
                duel::swap_control(d, c, tc, 0, 0);
            }
        }
    }
}
