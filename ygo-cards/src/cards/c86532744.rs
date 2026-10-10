//! Number S39: Utopia Prime
use super::prelude::*;

pub const ID: u32 = 86532744;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(39), listed_names: &[84013237], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 4 LIGHT monsters OR 1 "Number 39: Utopia" you control
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT),4,3,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, sc, st, Some(tp)))),
        Some(4),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Destroy as many Special Summoned monsters your opponent controls as possible, and if you do, banish them, then inflict 300 damage to your opponent for each monster banished
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e,tp) return Duel.GetLP(1-tp)>=Duel.GetLP(tp)+3000 end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(3),Cost.PayLP(10,true)))
    effect::set_cost(
        d,
        e1,
        Some(cost::and(vec![cost::detach_from_self(cost::Count::N(3), None, None), cost::pay_lp(cost::PayLp::Until(10))])),
    );
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,84013237) and c:IsFaceup()
    card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[84013237]) && card::is_faceup(d, c)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned() and c:IsAbleToRemove()
    card::is_special_summoned(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// the condition of e1: `Duel.GetLP(1-tp)>=Duel.GetLP(tp)+3000`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, 1 - a.tp) >= duel::get_lp(d, a.tp) + 3000
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (g.len() * 300) as i32);
    true
}
/// `s.rmctfilter(c)`
fn rmctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_REMOVED) and not c:IsReason(REASON_REDIRECT)
    card::is_location(d, c, LOCATION_REMOVED) && !card::is_reason(d, c, REASON_REDIRECT)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT,LOCATION_REMOVED)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, Some(LOCATION_REMOVED as u32), None) > 0 {
        // local ct=Duel.GetOperatedGroup():FilterCount(s.rmctfilter,nil)
        let operated = duel::get_operated_group(d);
        let ct = group::filter_count(d, &operated, &|d, c| rmctfilter(d, c), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,ct*300,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, (ct * 300) as i64, REASON_EFFECT, false, None);
        }
    }
}
