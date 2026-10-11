//! Constellar Meteor
use super::prelude::*;

pub const ID: u32 = 63545861;

// s.listed_series={SET_CONSTELLAR}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CONSTELLAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE);
    }
    true
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d);
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if a:IsSetCard(SET_CONSTELLAR) and d and d:IsControler(1-tp) and d:IsRelateToBattle() then g:AddCard(d) end
    let a_card = att.nil();
    if card::is_set_card(d, a_card, &[SET_CONSTELLAR as u16]) {
        if let Some(dc) = dd {
            if card::is_controler(d, dc, 1 - tp) && card::is_relate_to_battle(d, dc) {
                group::add_card(&mut g, &CardSet::from_card(dc));
            }
        }
    }
    // if d and d:IsSetCard(SET_CONSTELLAR) and a:IsControler(1-tp) and a:IsRelateToBattle() then g:AddCard(a) end
    if let Some(dc) = dd {
        if card::is_set_card(d, dc, &[SET_CONSTELLAR as u16]) && card::is_controler(d, a_card, 1 - tp) && card::is_relate_to_battle(d, a_card) {
            group::add_card(&mut g, &CardSet::from_card(a_card));
        }
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
