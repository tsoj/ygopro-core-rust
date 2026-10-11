//! Imprudent Intrusion
//! 死償不知
use super::prelude::*;

pub const ID: u32 = 26357901;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
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
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)<Duel.GetLP(1-tp)
    duel::get_lp(d, a.tp) < duel::get_lp(d, 1 - a.tp)
}
/// `s.desfilter(c,lp)`
fn desfilter(d: &mut Duel, c: CardId, lp: i32) -> bool {
    // return c:IsFaceup() and c:IsAttackBelow(lp)
    card::is_faceup(d, c) && card::is_attack_below(d, c, lp)
}
/// `s.spfilter(c,e,tp,lp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lp: i32) -> bool {
    // return c:IsMonster() and c:IsAttackBelow(lp) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_monster(d, c) && card::is_attack_below(d, c, lp) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local lp=Duel.GetLP(1-tp)-Duel.GetLP(tp)
    let lp = duel::get_lp(d, 1 - tp) - duel::get_lp(d, tp);
    // local g1=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,lp)
    let g1 = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, lp)), tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp,lp)
    let g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, lp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return lp>0 and (#g1>0 or (#g2>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0)) end
    if a.chk == Some(0) {
        return lp > 0
            && (!g1.is_empty() || (!g2.is_empty() && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0));
    }
    // local b1=#g1>0
    let b1 = !g1.is_empty();
    // local b2=#g2>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let b2 = !g2.is_empty() && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,0)},
    //     {b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 1, 0, 0);
    } else {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g2,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g2), 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local lp=Duel.GetLP(1-tp)-Duel.GetLP(tp)
    let lp = duel::get_lp(d, 1 - tp) - duel::get_lp(d, tp);
    // if lp<=0 then return end
    if lp <= 0 {
        return;
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,0,LOCATION_MZONE,1,1,nil,lp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, lp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp,lp)
        let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp, lp));
        let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
