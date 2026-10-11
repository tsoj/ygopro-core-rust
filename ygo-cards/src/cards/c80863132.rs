//! Muko
use super::prelude::*;

pub const ID: u32 = 80863132;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re:IsHasProperty(EFFECT_FLAG_PLAYER_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_PLAYER_TARGET, 0) {
        return false;
    }
    // if Duel.GetOperationCount(ev)~=1 then return false end
    if duel::get_operation_count(d, a.ev) != 1 {
        return false;
    }
    // local ex,cg,cc,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_DRAW)
    // return ex and cv>0
    match duel::get_operation_info(d, a.ev, CATEGORY_DRAW) {
        None => false,
        Some(info) => info.op_param > 0,
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.ChangeChainOperation(ev,s.repop)
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // local ex,cg,cc,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_DRAW)
    let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_DRAW) else {
        // cp is nil here: `cp<2` is a Lua error
        crate::lua_error("attempt to compare nil with number")
    };
    // if cp<2 then
    if info.op_player < 2 {
        // Duel.DiscardDeck(cp,cv,REASON_EFFECT)
        duel::discard_deck(d, info.op_player, info.op_param as u16, REASON_EFFECT);
    }
}
