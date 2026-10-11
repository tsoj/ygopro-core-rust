//! Utopia Buster
use super::prelude::*;

pub const ID: u32 = 30341772;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_UTOPIA)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UTOPIA as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local tg=g:GetMinGroup(Card.GetAttack)
    let tg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|x| x.0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // local tc=nil
        // local tg=g:GetMinGroup(Card.GetAttack)
        let tg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|x| x.0).nil();
        let tc: CardId;
        // if #tg>1 then
        if tg.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local sg=tg:Select(tp,1,1,nil)
            let sg = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // tc=sg:GetFirst()
            tc = group::get_first(d, &sg).nil();
        } else {
            // tc=tg:GetFirst()
            tc = group::get_first(d, &tg).nil();
        }
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // if Duel.Destroy(tc,REASON_EFFECT)>0 then
        if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
