//! Frontier Wiseman
use super::prelude::*;

pub const ID: u32 = 38742075;

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
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_WARRIOR) and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE)
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetCardTargetCount()>0 and c:IsSpell()
    //   and c:GetCardTarget():IsExists(s.cfilter,1,nil,e:GetHandlerPlayer())
    if card::get_card_target_count(d, c) == 0 || !card::is_spell(d, c) {
        return false;
    }
    let hp = effect::get_handler_player(d, e);
    let g = card::get_card_target(d, c);
    group::is_exists(d, &g, &|d, x| cfilter(d, x, hp), 1, NO_EX)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    // if not re:IsSpellEffect() then return end
    if !effect::is_spell_effect(d, re) {
        return;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = match info.into_iter().next() {
        Some(Value::Group(g)) => Some(g),
        _ => None,
    };
    // if not g or not g:IsExists(s.cfilter,1,nil,tp) then return end
    let Some(g) = g else {
        return;
    };
    if !group::is_exists(d, &g, &|d, c| cfilter(d, c, tp), 1, NO_EX) {
        return;
    }
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
            duel::destroy(d, h, REASON_EFFECT, None, None);
        }
    }
}
