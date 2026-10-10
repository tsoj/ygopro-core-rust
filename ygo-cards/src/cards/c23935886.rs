//! Draco Masters of the Tenyi
use super::prelude::*;

pub const ID: u32 = 23935886;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,3,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))), None, None);
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(aux.TargetBoolFunction(Card.IsType,TYPE_EFFECT))
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_EFFECT)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_LINK,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_LINK, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.desfilter = aux.FaceupFilter(Card.IsType,TYPE_EFFECT)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    !duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE)) and c:IsNonEffectMonster()
    (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE)) && card::is_non_effect_monster(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_matching_card(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,0,LOCATION_MZONE,1,ct,nil)
        let g = duel::select_matching_card(d, tp, Some(&desfilter), tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
