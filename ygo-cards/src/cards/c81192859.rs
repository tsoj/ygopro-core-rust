//! Gagaku-P.U.N.K. Wild Picking
//! scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 81192859;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Pop on attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
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
    // Protect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.pcon)
    effect::set_condition(d, e3, Some(condition(pcon)));
    // e3:SetOperation(s.pop)
    effect::set_operation(d, e3, Some(operation(pop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `local a,at=Duel.GetAttacker(),Duel.GetAttackTarget(); if a:IsControler(1-tp) then a,at=at,a end`
fn battle_pair(d: &mut Duel, tp: u8) -> (Option<CardId>, Option<CardId>) {
    let mut a = duel::get_attacker(d);
    let mut at = duel::get_attack_target(d);
    if card::is_controler(d, a.nil(), 1 - tp) {
        std::mem::swap(&mut a, &mut at);
    }
    (a, at)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,at=... (see battle_pair)
    let (x, at) = battle_pair(d, a.tp);
    // return a and at and a:IsSetCard(SET_PUNK) and a:IsFaceup() and at:IsControler(1-tp)
    match (x, at) {
        (Some(x), Some(at)) => card::is_set_card(d, x, &[SET_PUNK as u16]) && card::is_faceup(d, x) && card::is_controler(d, at, 1 - a.tp),
        _ => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local a,at=...
    let (_, at) = battle_pair(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,at,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at.map(CardSet::from_card), 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local a,at=...
    let (_, at) = battle_pair(d, a.tp);
    // if at and at:IsRelateToBattle() and at:IsControler(1-tp) then Duel.Destroy(at,REASON_EFFECT) end
    if let Some(at) = at {
        if card::is_relate_to_battle(d, at) && card::is_controler(d, at, 1 - a.tp) {
            duel::destroy(d, CardSet::from_card(at), REASON_EFFECT, None, None);
        }
    }
}
/// `s.pcon(e,tp,eg,ep,ev,re,r,rp)`
fn pcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_SZONE)
    a.rp == 1 - a.tp && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_controler(d, c, a.tp) && card::is_previous_location(d, c, LOCATION_SZONE)
}
/// `s.pop(e,tp,eg,ep,ev,re,r,rp)`
fn pop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_PUNK))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_PUNK as u16]))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
