//! Generaider Boss Room
//! 王の支配
//! original script by andré
use super::prelude::*;

pub const ID: u32 = 64325438;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // instant(chain)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `Card.IsDiscardable` as a filter (no arguments: reason REASON_COST)
fn is_discardable_cost(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&is_discardable_cost), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&is_discardable_cost), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ch=Duel.GetCurrentChain(true)-1
    let ch = duel::get_current_chain(d, true) - 1;
    // return ep==1-tp and ch>0 and Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_CONTROLER)==tp
    //     and Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_EFFECT):GetHandler():IsSetCard(SET_GENERAIDER)
    if !(a.ep == 1 - a.tp && ch > 0) {
        return false;
    }
    let info = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_EFFECT]);
    if info[0].int() != a.tp as i64 {
        return false;
    }
    let te = info[1].as_effect().nil();
    let h = effect::get_handler(d, te).nil();
    card::is_set_card(d, h, &[SET_GENERAIDER as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.repop)
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
    // if e:IsSpellTrapEffect() and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_spell_trap_effect(d, a.e) && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
