//! Collected Power
use super::prelude::*;

pub const ID: u32 = 7565547;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_EQUIP),tp,LOCATION_SZONE,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP)),
                tp,
                LOCATION_SZONE,
                LOCATION_SZONE,
                1,
                NO_EX,
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local dg=Group.CreateGroup()
    let mut dg = CardSet::new();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_EQUIP),tp,LOCATION_SZONE,LOCATION_SZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP)), tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    // for ec in g:Iter() do
    for ec in group::iter(d, &g) {
        // if tc:IsFaceup() and tc:IsRelateToEffect(e) and ec:CheckEquipTarget(tc) then
        let tc = tc.nil();
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::check_equip_target(d, ec, tc) {
            // Duel.Equip(tp,ec,tc,false,false)
            duel::equip(d, tp, ec, tc, false, false);
        } else {
            // dg:AddCard(ec)
            group::add_card(&mut dg, &CardSet::from_card(ec));
        }
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
