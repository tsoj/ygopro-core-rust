//! Wavering Eyes
use super::prelude::*;

pub const ID: u32 = 31222701;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE + CATEGORY_SEARCH);
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
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_PZONE,LOCATION_PZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_PZONE, LOCATION_PZONE) > 0;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,LOCATION_PZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, LOCATION_PZONE);
    let n = g.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
/// `s.thfilter1(c)`
fn thfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.thfilter2(c)`
fn thfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsAbleToHand()
    card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,LOCATION_PZONE)
    let g = duel::get_field_group(d, tp, LOCATION_PZONE, LOCATION_PZONE);
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if ct>=1 then
    if ct >= 1 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,500,REASON_EFFECT)
        duel::damage(d, 1 - tp, 500, REASON_EFFECT, false, None);
    }
    // local hg1=Duel.GetMatchingGroup(s.thfilter1,tp,LOCATION_DECK,0,nil)
    let hg1 = duel::get_matching_group(d, Some(&thfilter1), tp, LOCATION_DECK, 0, NO_EX);
    // if ct>=2 and #hg1>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if ct >= 2 && !hg1.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local shg1=hg1:Select(tp,1,1,nil)
        let shg1 = group::select(d, &hg1, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(shg1,nil,REASON_EFFECT)
        duel::send_to_hand(d, shg1.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,shg1)
        duel::confirm_cards(d, 1 - tp, shg1);
    }
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if ct>=3 and #rg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if ct >= 3 && !rg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local srg=rg:Select(tp,1,1,nil)
        let srg = group::select(d, &rg, tp, 1, 1, false, NO_EX).nil();
        // Duel.Remove(srg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, srg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // local hg2=Duel.GetMatchingGroup(s.thfilter2,tp,LOCATION_DECK,0,nil)
    let hg2 = duel::get_matching_group(d, Some(&thfilter2), tp, LOCATION_DECK, 0, NO_EX);
    // if ct==4 and #hg2>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if ct == 4 && !hg2.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local shg2=hg2:Select(tp,1,1,nil)
        let shg2 = group::select(d, &hg2, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(shg2,nil,REASON_EFFECT)
        duel::send_to_hand(d, shg2.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,shg2)
        duel::confirm_cards(d, 1 - tp, shg2);
    }
}
