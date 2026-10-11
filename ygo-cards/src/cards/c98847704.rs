//! Release Restraint Wave
use super::prelude::*;

pub const ID: u32 = 98847704;

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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EQUIP) and (not c:IsTrap() or c:IsPreviousLocation(LOCATION_MZONE))
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EQUIP)
        && (!card::is_trap(d, c) || card::is_previous_location(d, c, LOCATION_MZONE))
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(tp) and s.filter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, a.tp) && filter1(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_SZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), a.tp, LOCATION_SZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter2), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter1), a.tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX);
    // local dg=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_SZONE,nil)
    let mut dg = duel::get_matching_group(d, Some(&filter2), a.tp, 0, LOCATION_SZONE, NO_EX);
    // dg:Merge(g)
    group::add_card(&mut dg, &g.nil());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let count = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), count, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // local dg=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_SZONE,nil)
            let mut dg = duel::get_matching_group(d, Some(&filter2), a.tp, 0, LOCATION_SZONE, NO_EX);
            // dg:AddCard(tc)
            group::add_card(&mut dg, &CardSet::from_card(tc));
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
