//! Unification
use super::prelude::*;

pub const ID: u32 = 95352218;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // adjust
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetOperation(s.adjustop)
    effect::set_operation(d, e2, Some(operation(adjustop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.checklv(g)
fn checklv(d: &mut Duel, g: &CardSet) -> bool {
    // local tc=g:GetFirst()
    let tc = group::get_first(d, g).nil();
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // tc=g:GetNext()
    let _next = group::iter(d, g).get(1).copied();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, g) {
        // if tc:GetLevel()~=lv then return false end
        if card::get_level(d, tc) != lv {
            return false;
        }
    }
    // return true
    true
}
// function s.filter2(c,lv)
fn filter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and c:GetLevel()~=lv
    card::is_faceup(d, c) && card::get_level(d, c) != lv
}
// function s.adjustop(e,tp,eg,ep,ev,re,r,rp)
fn adjustop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local turnp=Duel.GetTurnPlayer()
    let turnp = duel::get_turn_player(d);
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelAbove,1),turnp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_level_above(d, c, 1)), turnp, LOCATION_MZONE, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // if s.checklv(g) then return end
    if checklv(d, &g) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,turnp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, turnp, aux::stringid(ID, 1));
    // local clv=g:Select(turnp,1,1,nil):GetFirst():GetLevel()
    let sel = group::select(d, &g, turnp, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &sel).nil();
    let clv = card::get_level(d, first);
    // local dg=g:Filter(s.filter2,nil,clv)
    let dg = group::filter(d, &g, &|d, c| filter2(d, c, clv), NO_EX);
    // Duel.SendtoGrave(dg,REASON_EFFECT)
    duel::send_to_grave(d, dg, REASON_EFFECT, None, None);
}
