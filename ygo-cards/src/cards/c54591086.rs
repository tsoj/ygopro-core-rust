//! Virus Cannon
use super::prelude::*;

pub const ID: u32 = 54591086;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TOKEN)
    !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.rfilter,1,false,nil,nil)
        return duel::check_release_group_cost(d, tp, Some(&rfilter), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.rfilter,1,1<<52,false,nil,nil)
    let rg = duel::select_release_group_cost(d, tp, Some(&rfilter), 1, 1 << 52, false, None, NO_EX);
    // e:SetLabel(#rg)
    effect::set_label(d, e, &[rg.len() as i64]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToGrave()
    card::is_spell(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u16;
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(1-tp,s.filter,1-tp,LOCATION_DECK,0,ct,ct,nil)
    let g = duel::select_matching_card(d, 1 - tp, Some(&filter), 1 - tp, LOCATION_DECK, 0, ct, ct, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
    }
}
