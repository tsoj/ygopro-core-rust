//! Criosphinx
use super::prelude::*;

pub const ID: u32 = 18654201;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // reg
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetTarget(s.hdtg)
    effect::set_target(d, e2, Some(cost_target(hdtg)));
    // e2:SetOperation(s.hdop)
    effect::set_operation(d, e2, Some(operation(hdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,tp)`: `return c:IsControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsMonster()`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_monster(d, c)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    // local p1=false local p2=false
    // if eg:IsExists(s.filter,1,nil,0) then p1=true end
    let eg = a.eg.clone().nil();
    let p1 = group::is_exists(d, &eg, &|d, c| filter(d, c, 0), 1, NO_EX);
    // if eg:IsExists(s.filter,1,nil,1) then p2=true end
    let p2 = group::is_exists(d, &eg, &|d, c| filter(d, c, 1), 1, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if p1 and p2 then
    if p1 && p2 {
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,r,rp,PLAYER_ALL,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r, a.rp, PLAYER_ALL as u8, 0);
    } else if p1 {
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,r,rp,0,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 0, 0);
    } else if p2 {
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,r,rp,1,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 1, 0);
    }
}
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, c, a.e);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,ep,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.ep, LOCATION_HAND as i32);
    true
}
fn hdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // if ep==PLAYER_ALL then
    if a.ep == PLAYER_ALL as u8 {
        // Duel.DiscardHand(0,nil,1,1,REASON_EFFECT)
        duel::discard_hand(d, 0, None, 1, 1, REASON_EFFECT, NO_EX);
        // Duel.DiscardHand(1,nil,1,1,REASON_EFFECT)
        duel::discard_hand(d, 1, None, 1, 1, REASON_EFFECT, NO_EX);
    } else {
        // Duel.DiscardHand(ep,nil,1,1,REASON_EFFECT)
        duel::discard_hand(d, a.ep, None, 1, 1, REASON_EFFECT, NO_EX);
    }
}
