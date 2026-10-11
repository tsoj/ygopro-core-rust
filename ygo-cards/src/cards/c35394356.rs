//! Daruma Dropper
use super::prelude::*;

pub const ID: u32 = 35394356;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reduce an opponent's monster ATK to 0 for a turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Except turn was sent to GY, banish this card instead of detaching
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e2, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.rcon)
    effect::set_condition(d, e2, Some(condition(rcon)));
    // e2:SetOperation(s.rop)
    effect::set_operation(d, e2, Some(operation_count(rop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// Detach a material from your field
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_COST) end
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, a.tp, 1, 0, 1, REASON_COST, None);
    }
    // Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_COST)
    duel::remove_overlay_card(d, a.tp, 1, 0, 1, 1, REASON_COST, None);
    true
}
// Activation legality
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// Reduce ATK to 0 for this turn
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(0)
            effect::set_value(d, e1, 0);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
// Except turn was sent, and would detach
// function s.rcon(e,tp,eg,ep,ev,re,r,rp)
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.exccon(e) and (r&REASON_COST)~=0 and re:IsActivated()
    if !aux::exccon().call(d, &[Value::Effect(a.e)]).truthy() {
        return false;
    }
    if (a.r & REASON_COST) == 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_activated(d, re) {
        return false;
    }
    // and re:IsActiveType(TYPE_XYZ)
    if !effect::is_active_type(d, re, TYPE_XYZ) {
        return false;
    }
    // and e:GetHandler():IsAbleToRemoveAsCost()
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_able_to_remove_as_cost(d, c, None) {
        return false;
    }
    // and ep==e:GetOwnerPlayer() and ev>=1
    if a.ep != effect::get_owner_player(d, a.e) || (a.ev as i64) < 1 {
        return false;
    }
    // and re:GetHandler():GetOverlayCount()>=ev-1
    let rc = effect::get_handler(d, re).nil();
    card::get_overlay_count(d, rc) as i64 >= a.ev as i64 - 1
}
// Detach substitution
// function s.rop(e,tp,eg,ep,ev,re,r,rp)
fn rop(d: &mut Duel, a: &EffArgs) -> i64 {
    // return Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_COST, None, None) as i64
}
