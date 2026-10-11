//! That Wacky Magic!
//! 暴走する魔力
use super::prelude::*;

pub const ID: u32 = 50427388;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    // return true
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToRemove()
    card::is_spell(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.filter(c,def)`
fn filter(d: &mut Duel, c: CardId, def: i32) -> bool {
    // return c:IsFaceup() and c:IsDefenseBelow(def)
    card::is_faceup(d, c) && card::is_defense_below(d, c, def)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    //     if e:GetLabel()~=1 then return false end
    //     e:SetLabel(0)
    //     local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    //     return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil,ct*300)
    // end
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        let ct = duel::get_matching_group_count(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_GRAVE, 0, NO_EX) as i32;
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ct * 300)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    let n = g.len() as i32;
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(#g*300)
    effect::set_label(d, a.e, &[(n * 300) as i64]);
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,#g*300)
    let sg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, n * 300)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let count = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), count, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,e:GetLabel())
    let label = effect::get_label(d, a.e) as i32;
    let sg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, label)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
