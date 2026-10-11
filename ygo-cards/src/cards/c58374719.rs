//! World Legacy Struggle
//! 星遺物への抵抗
use super::prelude::*;

pub const ID: u32 = 58374719;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetMutualLinkedGroupCount()>0
    card::get_mutual_linked_group_count(d, c) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, cc| card::is_spell_trap(d, cc)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local sg=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,e:GetHandler())
    let sg = duel::select_target(d, a.tp, Some(&|d, cc| card::is_spell_trap(d, cc)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct as u16, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,LOCATION_ONFIELD)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, LOCATION_ONFIELD as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
