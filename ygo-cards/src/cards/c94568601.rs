//! Tyrant Dragon
use super::prelude::*;

pub const ID: u32 = 94568601;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // self destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e3, LOCATION_SZONE, LOCATION_SZONE);
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // multiatk
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e4, EFFECT_EXTRA_ATTACK);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // spsummon cost
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e5, EFFECT_SPSUMMON_COST);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e5, LOCATION_GRAVE);
    // e5:SetCost(s.spcost)
    effect::set_cost(d, e5, Some(lua_fn(spcost)));
    // e5:SetOperation(s.spcop)
    effect::set_operation(d, e5, Some(operation(spcop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if not c:IsTrap() or c:GetCardTargetCount()==0 then return false end
    if !card::is_trap(d, c) || card::get_card_target_count(d, c) == 0 {
        return false;
    }
    // return c:GetCardTarget():IsContains(e:GetHandler())
    let targets = card::get_card_target(d, c);
    let h = effect::get_handler(d, e);
    h.is_some_and(|h| group::is_contains(&targets, h))
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if not re:IsTrapEffect() then return end
    if !effect::is_trap_effect(d, re) {
        return;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // if not g or #g==0 then return end
    let Some(g) = g else { return };
    if g.is_empty() {
        return;
    }
    // if g:IsContains(e:GetHandler()) then
    let h = effect::get_handler(d, a.e);
    if h.is_some_and(|h| group::is_contains(&g, h)) {
        // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
        if duel::negate_effect(d, a.ev as u8) {
            let rc = effect::get_handler(d, re).nil();
            if card::is_relate_to_effect(d, rc, re) {
                // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
                duel::destroy(d, rc, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_MZONE)>0
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) > 0
}
/// `s.spcost(e,c,tp)`
fn spcost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    // return Duel.CheckReleaseGroupCost(tp,Card.IsRace,1,false,nil,nil,RACE_DRAGON)
    duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_race(d, c, RACE_DRAGON)), 1, 1, false, None, NO_EX)
}
/// `s.spcop(e,tp,eg,ep,ev,re,r,rp)`
fn spcop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsRace,1,1,false,nil,nil,RACE_DRAGON)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_race(d, c, RACE_DRAGON)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_EFFECT)
    duel::release(d, g, REASON_EFFECT, None);
}
