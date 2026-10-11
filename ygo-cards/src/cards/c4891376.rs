//! Zalen the Shackled Dragon
use super::prelude::*;

pub const ID: u32 = 4891376;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ Synchro Monsters
    // Synchro.AddProcedure(c,nil,1,1,aux.FilterBoolFunctionEx(Card.IsType,TYPE_SYNCHRO),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))), 1, 99, None, None, None, None, None);
    // Multiple Tuner check (Red Nova)
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.valcheck)
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If this card you control would be used as Synchro Material, you can treat it as a non-Tuner
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_NONTUNER)
    effect::set_code(d, e1, EFFECT_NONTUNER);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function() return Duel.GetCurrentChain()>1 end)
    effect::set_condition(d, e2, Some(condition(chain_cond)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.GetCurrentChain()>1 end`
fn chain_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_current_chain(d, false) > 1
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,2,nil,TYPE_TUNER) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_TUNER), 2, NO_EX) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetCode(EFFECT_MULTIPLE_TUNERS)
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD)|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    0
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let ev = a.ev as i64;
    // local b1=Duel.IsChainDisablable(ev)
    let b1 = duel::is_chain_disablable(d, ev as u8);
    // local b2=Duel.IsChainDisablable(ev-1)
    let b2 = duel::is_chain_disablable(d, (ev - 1) as u8);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as i64;
    // local ch=ev-op+1
    let ch = ev - op + 1;
    // e:SetLabel(ch)
    effect::set_label(d, a.e, &[ch]);
    // local eff=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_EFFECT)
    let eff = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local rc=eff:GetHandler()
    let rc = effect::get_handler(d, eff).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,rc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(eff) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, eff) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,rc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local ch=e:GetLabel()
    let ch = effect::get_label(d, a.e);
    // local eff=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_EFFECT)
    let eff = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local rc=eff:GetHandler()
    let rc = effect::get_handler(d, eff).nil();
    // if Duel.NegateEffect(ch) and rc:IsRelateToEffect(eff) then
    if duel::negate_effect(d, ch as u8) && card::is_relate_to_effect(d, rc, eff) {
        // Duel.Destroy(rc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(rc), REASON_EFFECT, None, None);
    }
}
