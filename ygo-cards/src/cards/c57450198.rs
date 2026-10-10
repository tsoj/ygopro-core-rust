//! Super Quantal Mech Beast Lusterrex
use super::prelude::*;

pub const ID: u32 = 57450198;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPER_QUANTUM], listed_names: &[73422829], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // Cannot attack unless it has Xyz Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayCount()==0 end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            card::get_overlay_count(d, c) == 0
        })),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the effects of a face-up monster until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetCondition(aux.NOT(s.quickeffcond))
    effect::set_condition(d, e2, Some(condition(|d, a| !quickeffcond(d, a))));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // This is a Quick effect if this card has "Super Quantum White Layer" as Xyz Material
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(s.quickeffcond)
    effect::set_condition(d, e3, Some(condition(quickeffcond)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Attach 1 "Super Quantum" monster from your hand or field to this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.attachtg)
    effect::set_target(d, e4, Some(cost_target(attachtg)));
    // e4:SetOperation(s.attachop)
    effect::set_operation(d, e4, Some(operation(attachop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.quickeffcond(e,tp,eg,ep,ev,re,r,rp)`
fn quickeffcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,73422829)
    let c = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[73422829]), 1, NO_EX)
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and not c:IsDisabled()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && !card::is_disabled(d, c)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return s.disfilter(chkc) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return disfilter(d, chkc) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // Duel.SelectTarget(tp,s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&disfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsDisabled() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_disabled(d, tc) {
        // tc:NegateEffects(c,RESETS_STANDARD_PHASE_END)
        card::negate_effects(d, tc, c, Some(RESETS_STANDARD_PHASE_END), false, None);
    }
}
/// `s.attachfilter(c,e,tp,xc)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8, xc: CardId) -> bool {
    // return c:IsSetCard(SET_SUPER_QUANTUM) and c:IsMonster() and not c:IsType(TYPE_TOKEN)
    //     and (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsCanBeXyzMaterial(xc,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_SUPER_QUANTUM as u16])
        && card::is_monster(d, c)
        && !card::is_type(d, c, TYPE_TOKEN)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,e,tp,c) end
    if a.chk == Some(0) {
        return card::is_type(d, c, TYPE_XYZ)
            && duel::is_existing_matching_card(d, Some(&|d, x| attachfilter(d, x, tp, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,e,tp,c):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| attachfilter(d, x, tp, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and not tc:IsImmuneToEffect(e) then
    if let Some(tc) = tc {
        if !card::is_immune_to_effect(d, tc, a.e) {
            // Duel.Overlay(c,tc,true)
            duel::overlay(d, c, tc, true);
        }
    }
}
