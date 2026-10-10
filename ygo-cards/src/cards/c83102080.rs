//! Parallel Twister
use super::prelude::*;

pub const ID: u32 = 83102080;

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
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToGraveAsCost()
    //   and Duel.IsExistingTarget(s.tgfilter,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ec,c)
    card::is_spell_trap(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, c)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ec.into())
}
fn tgfilter(_d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c~=tc
    c != tc
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && chkc != c;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=0 then
        if effect::get_label(d, e) != 0 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,0,1,c,c)
            return duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, c)), tp, LOCATION_ONFIELD, 0, 1, c.into());
        } else {
            // return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
            return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
        }
    }
    // if e:GetLabel()~=0 then
    if effect::get_label(d, e) != 0 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_ONFIELD,0,1,1,c,c)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, c)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc.nil(), REASON_EFFECT, None, None);
    }
}
