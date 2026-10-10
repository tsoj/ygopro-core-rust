//! Kumongous, the Sticky String Kaiju
use super::prelude::*;

pub const ID: u32 = 29726552;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_KAIJU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1,e2=aux.AddKaijuProcedure(c)
    aux::add_kaiju_procedure(d, c);
    // When opponent normal or special summons a monster(s), negate its effect also it cannot attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsFaceup()
    card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,1,COUNTER_KAIJU,2,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 1, COUNTER_KAIJU as u16, 2, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,1,COUNTER_KAIJU,2,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 1, COUNTER_KAIJU as u16, 2, REASON_COST);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.filter,1,nil,1-tp) and not eg:IsContains(e:GetHandler()) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let ok = group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - tp), 1, NO_EX);
        let h = effect::get_handler(d, a.e).nil();
        return ok && !group::is_contains(&eg, h);
    }
    // local g=eg:Filter(s.filter,nil,1-tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, 1 - tp), NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // --Cannot attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3206)
        effect::set_description(d, e1, 3206);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // --Negate their effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e2, EFFECT_DISABLE);
        // e2:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 2);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        // e3:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 2);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
