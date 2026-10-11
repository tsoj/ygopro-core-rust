//! Absolute Crusader
use super::prelude::*;

pub const ID: u32 = 70832512;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy level 5 or higher monsters that are Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.descond)
    effect::set_condition(d, e1, Some(condition(descond)));
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(5)
    card::is_faceup(d, c) && card::is_level_above(d, c, 5)
}
/// `s.descond(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.desfilter,1,e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &desfilter, 1, h.into())
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dg=eg:Filter(s.desfilter,e:GetHandler(),nil)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let dg = group::filter(d, &eg, &desfilter, h.into());
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, dg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local dg=eg:Filter(s.desfilter,nil):Filter(Card.IsRelateToEffect,nil,e)
    let eg = a.eg.clone().nil();
    let g1 = group::filter(d, &eg, &desfilter, NO_EX);
    let dg = group::filter(d, &g1, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
