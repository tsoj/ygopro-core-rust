//! Topologic Trisbaena
use super::prelude::*;

pub const ID: u32 = 72529749;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // --banish
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(aux.zptcon(nil))
    effect::set_condition(d, e1, Some(aux::zptcon(None)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=aux.zptgroup(eg,nil,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg();
    let mut g = aux::zptgroup(d, &eg, None, c, a.tp);
    // local tg=g:Filter(Card.IsLocation,nil,LOCATION_MZONE)
    let tg = group::filter(d, &g, &|d, x| card::is_location(d, x, LOCATION_MZONE), NO_EX);
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, x| card::is_spell_trap(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, x| card::is_spell_trap(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)~=0 then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
        // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsControler,nil,1-tp)
        let og = duel::get_operated_group(d);
        let ct = group::filter_count(d, &og, &|d, x| card::is_controler(d, x, 1 - a.tp), NO_EX);
        // Duel.Damage(1-tp,ct*500,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, (ct * 500) as i64, REASON_EFFECT, false, None);
    }
}
