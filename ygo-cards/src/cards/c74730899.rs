//! Batteryman Fuel Cell
use super::prelude::*;

pub const ID: u32 = 74730899;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTERYMAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // return hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.retcost)
    effect::set_cost(d, e2, Some(cost_target(retcost)));
    // e2:SetTarget(s.rettg)
    effect::set_target(d, e2, Some(cost_target(rettg)));
    // e2:SetOperation(s.retop)
    effect::set_operation(d, e2, Some(operation(retop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BATTERYMAN)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BATTERYMAN as u16])
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_MZONE,0,2,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spfilter), tp, LOCATION_MZONE, 0, 2, NO_EX)
}
/// `s.retcfilter(c)`
fn retcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BATTERYMAN)
    card::is_set_card(d, c, &[SET_BATTERYMAN as u16])
}
/// `s.retfilter(c,e)`
fn retfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.retcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn retcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    // local dg=Duel.GetMatchingGroup(s.retfilter,tp,0,LOCATION_ONFIELD,nil,e)
    let dg = duel::get_matching_group(d, Some(&|d, cc| retfilter(d, cc, e)), tp, 0, LOCATION_ONFIELD, NO_EX);
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,s.retcfilter,1,false,aux.ReleaseCheckTarget,e:GetHandler(),dg)
        return aux::check_release_group_cost(
            d,
            tp,
            Some(&retcfilter),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
            c.into(),
        );
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.retcfilter,1,1,false,aux.ReleaseCheckTarget,e:GetHandler(),dg)
    let g = aux::select_release_group_cost(
        d,
        tp,
        Some(&retcfilter),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
        c.into(),
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsAbleToHand()
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    if a.chk == Some(0) {
        // return Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil)
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.retop(e)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
