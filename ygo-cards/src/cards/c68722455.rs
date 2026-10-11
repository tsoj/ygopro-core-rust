//! Red Ogre
use super::prelude::*;

pub const ID: u32 = 68722455;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToGraveAsCost()
    card::is_able_to_grave_as_cost(d, c)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // local rt=Duel.GetTargetCount(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let rt = duel::get_target_count(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND,0,1,rt,nil)
    let cg = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, rt as u16, false, NO_EX).nil();
    // Duel.SendtoGrave(cg,REASON_COST)
    duel::send_to_grave(d, cg.clone(), REASON_COST, None, None);
    // e:SetLabel(#cg)
    effect::set_label(d, a.e, &[cg.len() as i64]);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local eg=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,ct,nil)
    let eg = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_able_to_hand(d, c, None)),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        ct as u16,
        ct as u16,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,eg,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, eg, ct as u8, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp,chk)
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let tg = info[0].as_group().cloned().nil();
    // local rg=tg:Filter(Card.IsRelateToEffect,nil,e)
    let rg = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.SendtoHand(rg,nil,REASON_EFFECT)
        duel::send_to_hand(d, rg, None, REASON_EFFECT, None);
    }
}
