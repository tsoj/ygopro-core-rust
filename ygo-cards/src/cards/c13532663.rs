//! Dummy Golem
use super::prelude::*;

pub const ID: u32 = 13532663;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,0,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 0, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) or not c:IsRelateToEffect(e) or not c:IsAbleToChangeControler()
    //     or not Duel.IsExistingMatchingCard(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil) then return end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        || !card::is_relate_to_effect(d, c, a.e)
        || !card::is_able_to_change_controler(d, c)
        || !duel::is_existing_matching_card(d, Some(&card::is_able_to_change_controler), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectMatchingCard(1-tp,Card.IsAbleToChangeControler,1-tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, 1 - a.tp, Some(&card::is_able_to_change_controler), 1 - a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SwapControl(c,g:GetFirst(),0,0)
    let tc = group::get_first(d, &g).nil();
    duel::swap_control(d, c, tc, 0, 0);
}
