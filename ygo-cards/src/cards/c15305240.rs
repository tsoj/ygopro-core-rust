//! Creature Seizure
//! 鹵獲装置
use super::prelude::*;

pub const ID: u32 = 15305240;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_NORMAL) and c:IsAbleToChangeControler()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_change_controler(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&card::is_able_to_change_controler), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,0,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 0, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //   or not Duel.IsExistingMatchingCard(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil)
    // then return end
    if !duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        || !duel::is_existing_matching_card(d, Some(&card::is_able_to_change_controler), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(g1)
    duel::hint_selection(d, g1.clone(), true);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CONTROL as u64);
    // local g2=Duel.SelectMatchingCard(1-tp,Card.IsAbleToChangeControler,1-tp,LOCATION_MZONE,0,1,1,nil)
    let g2 = duel::select_matching_card(d, 1 - a.tp, Some(&card::is_able_to_change_controler), 1 - a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(g2)
    duel::hint_selection(d, g2.clone(), true);
    // local c1=g1:GetFirst()
    let c1 = group::get_first(d, &g1).nil();
    // local c2=g2:GetFirst()
    let c2 = group::get_first(d, &g2).nil();
    // Duel.SwapControl(c1,c2,0,0)
    duel::swap_control(d, c1, c2, 0, 0);
}
