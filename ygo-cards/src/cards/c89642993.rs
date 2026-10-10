//! Number 63: Shamoji Soldier
use super::prelude::*;

pub const ID: u32 = 89642993;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(63), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,1,2)
    xyz::add_procedure(d, c, None, Some(1), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.eftg)
    effect::set_target(d, e1, Some(cost_target(eftg)));
    // e1:SetOperation(s.efop)
    effect::set_operation(d, e1, Some(operation(efop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    let op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE_START|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE_START | PHASE_STANDBY);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.drcon)
        effect::set_condition(d, e1, Some(condition(drcon)));
        // e1:SetOperation(s.drop)
        effect::set_operation(d, e1, Some(operation(drop_op)));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    } else {
        // Duel.Recover(tp,1000,REASON_EFFECT)
        duel::recover(d, tp, 1000, REASON_EFFECT, false, None);
        // Duel.Recover(1-tp,1000,REASON_EFFECT)
        duel::recover(d, 1 - tp, 1000, REASON_EFFECT, false, None);
    }
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, tp, 1, REASON_EFFECT);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - tp, 1, REASON_EFFECT);
}
