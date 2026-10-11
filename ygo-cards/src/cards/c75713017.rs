//! Prometeor, the Burning Star
use super::prelude::*;

pub const ID: u32 = 75713017;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // extra attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(condition(atcon)));
    // e1:SetCost(s.atcost)
    effect::set_cost(d, e1, Some(cost_target(atcost)));
    // e1:SetOperation(s.atop)
    effect::set_operation(d, e1, Some(operation(atop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atcon(e,tp,eg,ep,ev,re,r,rp)
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if not bc then return false end
    let Some(bc) = bc else {
        return false;
    };
    // local seq=bc:GetPreviousSequence()
    let seq = card::get_previous_sequence(d, bc);
    // e:SetLabel(seq+16)
    effect::set_label(d, a.e, &[seq as i64 + 16]);
    // return Duel.GetAttacker()==c and aux.bdocon(e,tp,eg,ep,ev,re,r,rp) and c:CanChainAttack(0) and seq<5
    if duel::get_attacker(d) != Some(c) {
        return false;
    }
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if !aux::bdocon().call(d, &args).truthy() {
        return false;
    }
    card::can_chain_attack(d, c, Some(0), false) && seq < 5
}
// function s.atcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_DISCARD|REASON_COST)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_DISCARD | REASON_COST, NO_EX);
    true
}
// function s.atop(e,tp,eg,ep,ev,re,r,rp)
fn atop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local seq=e:GetLabel()
    let seq = effect::get_label(d, a.e);
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    // e1:SetLabel(0x1<<seq)
    effect::set_label(d, e1, &[1i64 << seq]);
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(lua_fn(disop)));
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.disop(e,tp)
// return e:GetLabel()
fn disop(d: &mut Duel, a: Args) -> i64 {
    effect::get_label(d, a.e(0))
}
