//! Evilswarm Bahamut
use super::prelude::*;

pub const ID: u32 = 36757171;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LSWARM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_LSWARM),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_LSWARM as u16], sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // attack up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_CONTROL + CATEGORY_HANDES);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`: `return c:IsFaceup() and c:IsControlerCanBeChanged()`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.dfilter(c)`: `return c:IsSetCard(SET_LSWARM) and c:IsMonster() and c:IsDiscardable()`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LSWARM as u16]) && card::is_monster(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&dfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.dfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&dfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.GetControl(tc,tp)
        duel::get_control(d, CardSet::from_card(tc), tp, 0, 0, None, None);
    }
}
