//! Junk Breaker
use super::prelude::*;

pub const ID: u32 = 43436049;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.sumsuc)
    effect::set_operation(d, e1, Some(operation(sumsuc)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sumsuc(e,tp,eg,ep,ev,re,r,rp)`
fn sumsuc(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TEMP_REMOVE|RESET_TURN_SET)|RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(
        d,
        h,
        ID,
        RESET_EVENT | (RESETS_STANDARD & !(RESET_TEMP_REMOVE | RESET_TURN_SET)) | RESET_PHASE | PHASE_END,
        0,
        1,
        0,
        0,
    );
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler()) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_negatable_monster), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, h.into());
    }
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let g = duel::get_matching_group(d, Some(&card::is_negatable_monster), a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_negatable_monster), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // if tc:IsType(TYPE_TRAPMONSTER) then
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, h);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
}
