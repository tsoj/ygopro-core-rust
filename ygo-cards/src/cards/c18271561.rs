//! Chthonian Blast
use super::prelude::*;

pub const ID: u32 = 18271561;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsPreviousControler(tp) and c:IsReason(REASON_DESTROY)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_DESTROY)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter,1,nil,tp)
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| filter(d, c, tp), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local dg=g:GetMinGroup(Card.GetAttack)
    let dg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|x| x.0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, dg, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local dg=g:GetMinGroup(Card.GetAttack)
    let mut dg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|x| x.0).nil();
    // if #dg>1 then
    if dg.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // dg=dg:Select(tp,1,1,nil)
        dg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
    }
    // local atk=dg:GetFirst():GetAttack()/2
    let first = group::get_first(d, &dg).nil();
    let atk = card::get_attack(d, first) as f64 / 2.0;
    // if Duel.Destroy(dg,REASON_EFFECT)>0 then
    if duel::destroy(d, dg, REASON_EFFECT, None, None) > 0 {
        // Duel.Damage(tp,atk,REASON_EFFECT,true)
        duel::damage(d, tp, lua_int(atk), REASON_EFFECT, true, None);
        // Duel.Damage(1-tp,atk,REASON_EFFECT,true)
        duel::damage(d, 1 - tp, lua_int(atk), REASON_EFFECT, true, None);
        // Duel.RDComplete()
        duel::rd_complete(d);
    }
}
