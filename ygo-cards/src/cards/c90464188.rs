//! Obsidian Dragon
use super::prelude::*;

pub const ID: u32 = 90464188;

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
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
// function s.distg(e,c)
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if c:GetCardTargetCount()==0 then return false end
    if card::get_card_target_count(d, c) == 0 {
        return false;
    }
    // return c:GetCardTarget():IsExists(s.disfilter,1,nil,e:GetHandlerPlayer())
    let g = card::get_card_target(d, c);
    let p = effect::get_handler_player(d, e);
    group::is_exists(d, &g, &|d, x| disfilter(d, x, p), 1, NO_EX)
}
// s.disfilter(c,tp)
fn disfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsAttribute(ATTRIBUTE_DARK)
    card::is_controler(d, c, tp) && card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_attribute(d, c, ATTRIBUTE_DARK)
}
// s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not re:IsSpellTrapEffect() or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    let re = a.re.nil();
    if !effect::is_spell_trap_effect(d, re) || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not g or #g==0 then return end
    let Some(g) = g else {
        return;
    };
    if group::get_count(&g) == 0 {
        return;
    }
    // if g:IsExists(s.disfilter,1,nil,tp) then
    if group::is_exists(d, &g, &|d, x| disfilter(d, x, tp), 1, NO_EX) {
        // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
        if duel::negate_effect(d, a.ev as u8) {
            let h = effect::get_handler(d, re).nil();
            if card::is_relate_to_effect(d, h, re) {
                // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
                duel::destroy(d, h, REASON_EFFECT, None, None);
            }
        }
    }
}
