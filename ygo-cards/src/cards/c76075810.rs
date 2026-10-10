//! Throwstone Unit
use super::prelude::*;

pub const ID: u32 = 76075810;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.descost)
    effect::set_cost(d, e1, Some(cost_target(descost)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR)
    card::is_race(d, c, RACE_WARRIOR)
}
/// `s.desfilter(c,atk)`
fn desfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsDefenseBelow(atk)
    card::is_faceup(d, c) && card::is_defense_below(d, c, atk)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dg=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,c:GetAttack())
    let atk = card::get_attack(d, c);
    let dg = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,aux.ReleaseCheckTarget,c,dg) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(
            d,
            a.tp,
            Some(&cfilter),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
            c.into(),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,aux.ReleaseCheckTarget,c,dg)
    let g = aux::select_release_group_cost(
        d,
        a.tp,
        Some(&cfilter),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
        c.into(),
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g.clone(), REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg-g,1,0,0)
    let rest = group::difference(&dg, &g);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(rest), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=c:GetAttack()
    let mut atk = card::get_attack(d, c);
    // if not c:IsRelateToEffect(e) then atk=c:GetPreviousAttackOnField() end
    if !card::is_relate_to_effect(d, c, a.e) {
        atk = card::get_previous_attack_on_field(d, c);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,atk)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| desfilter(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
