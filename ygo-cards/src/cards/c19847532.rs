//! Infernal Flame Emperor
use super::prelude::*;

pub const ID: u32 = 19847532;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local dg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // local ct=#dg
    let mut ct = dg.len();
    // if ct>5 then ct=5 end
    if ct > 5 {
        ct = 5;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,ct,nil)
    let rg = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, ct as u16, false, NO_EX).nil();
    let rn = rg.len();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.SetTargetParam(#rg)
    duel::set_target_param(d, rn as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#rg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), rn as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ct = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct, ct, false, NO_EX).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
