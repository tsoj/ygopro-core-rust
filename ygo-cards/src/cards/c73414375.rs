//! Dimension Jar
use super::prelude::*;

pub const ID: u32 = 73414375;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,3,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 3, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToRemove(tp) and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, Some(tp), None, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // if Duel.IsExistingTarget(s.rmfilter,p,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil,1-p) and Duel.SelectYesNo(p,aux.Stringid(id,1)) then
    if duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, 1 - p)), p, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, p, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_REMOVE as u64);
        // local rg=Duel.SelectMatchingCard(p,s.rmfilter,p,0,LOCATION_MZONE|LOCATION_GRAVE,1,3,nil,p)
        let rg = duel::select_matching_card(d, p, Some(&|d, c| rmfilter(d, c, p)), p, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 3, false, NO_EX).nil();
        // Duel.HintSelection(rg,true)
        duel::hint_selection(d, rg.clone(), true);
        // g:Merge(rg)
        group::add_card(&mut g, &rg);
    }
    // if Duel.IsExistingTarget(s.rmfilter,1-p,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil,1-p) and Duel.SelectYesNo(1-p,aux.Stringid(id,1)) then
    if duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, 1 - p)), 1 - p, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, 1 - p, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,1-p,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - p, HINTMSG_REMOVE as u64);
        // local rg=Duel.SelectMatchingCard(1-p,s.rmfilter,1-p,0,LOCATION_MZONE|LOCATION_GRAVE,1,3,nil,1-p)
        let rg = duel::select_matching_card(d, 1 - p, Some(&|d, c| rmfilter(d, c, 1 - p)), 1 - p, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 3, false, NO_EX).nil();
        // Duel.HintSelection(rg,true)
        duel::hint_selection(d, rg.clone(), true);
        // g:Merge(rg)
        group::add_card(&mut g, &rg);
    }
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
