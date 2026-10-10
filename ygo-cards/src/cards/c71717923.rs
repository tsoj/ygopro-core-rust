//! Mormolith
use super::prelude::*;

pub const ID: u32 = 71717923;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost_target(descost)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_EARTH)
    //     and Duel.IsExistingMatchingCard(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,c,c:GetAttack())
    if !(card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH)) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, atk)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.filter(c,atk)`
fn filter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsDefenseBelow(atk)
    card::is_faceup(d, c) && card::is_defense_below(d, c, atk)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    // e:SetLabel(g:GetFirst():GetAttack())
    let first = group::get_first(d, &g).nil();
    let atk = card::get_attack(d, first);
    effect::set_label(d, a.e, &[atk as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,nil,e:GetLabel())
    let atk = effect::get_label(d, a.e) as i32;
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, atk)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,nil,e:GetLabel())
    let atk = effect::get_label(d, a.e) as i32;
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, atk)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
