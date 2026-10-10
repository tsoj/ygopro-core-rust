//! Teardrop the Rikka Queen
//! 六花聖ティアドロップ
use super::prelude::*;

pub const ID: u32 = 33779875;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 8 monsters
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // Tribute 1 monster on the field
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_RELEASE)
    effect::set_category(d, e1a, CATEGORY_RELEASE);
    // e1a:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1a, EFFECT_TYPE_IGNITION);
    // e1a:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1a, EFFECT_FLAG_CARD_TARGET, 0);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(aux.NOT(s.quickcon))
    effect::set_condition(d, e1a, Some(condition(|d, a| !quickcon(d, a))));
    // e1a:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1a, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1a:SetTarget(s.tribtg)
    effect::set_target(d, e1a, Some(cost_target(tribtg)));
    // e1a:SetOperation(s.tribop)
    effect::set_operation(d, e1a, Some(operation(tribop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1b, EFFECT_TYPE_QUICK_O);
    // e1b:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1b, EVENT_FREE_CHAIN);
    // e1b:SetCondition(s.quickcon)
    effect::set_condition(d, e1b, Some(condition(quickcon)));
    // e1b:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // This card gains 200 ATK for each monster Tributed, until the end of this turn
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2a, CATEGORY_ATKCHANGE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2a, EVENT_CUSTOM + ID);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetOperation(s.atkop)
    effect::set_operation(d, e2a, Some(operation(atkop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // Keep track of the number of Tributed monsters and raise the custom event using that number as the event value
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2b:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2b, EVENT_RELEASE);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetOperation(s.regop)
    effect::set_operation(d, e2b, Some(operation(regop)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.tribtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tribtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsReleasableByEffect() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_releasable_by_effect(d, chkc, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsReleasableByEffect,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsReleasableByEffect,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, g, 1, tp, 0);
    true
}
/// `s.tribop(e,tp,eg,ep,ev,re,r,rp)`
fn tribop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Release(tc,REASON_EFFECT)
        duel::release(d, tc, REASON_EFFECT, None);
    }
}
/// `s.quickcon(e,tp,eg,ep,ev,re,r,rp)`
fn quickcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsRace,1,nil,RACE_PLANT)
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d, c| card::is_race(d, c, RACE_PLANT as u64), 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // This card gains 200 ATK for each, until the end of this turn
        // c:UpdateAttack(ev*200,RESETS_STANDARD_DISABLE_PHASE_END)
        card::update_attack(d, c, a.ev as i64 * 200, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
    }
}
/// `s.atkconfilter(c)`
fn atkconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) or (not c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsMonster())
    card::is_previous_location(d, c, LOCATION_MZONE) || (!card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_monster(d, c))
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local ct=eg:FilterCount(s.atkconfilter,nil)
    let eg = a.eg();
    let ct = group::filter_count(d, &eg, &atkconfilter, NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,re,r,rp,ep,ct)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, a.re, a.r, a.rp, a.ep, ct as u32);
    }
}
