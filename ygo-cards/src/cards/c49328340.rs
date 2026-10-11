//! Spiral Spear Strike
use super::prelude::*;

pub const ID: u32 = 49328340;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[6368038, 16589042, CARD_GAIA_CHAMPION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // --If "Gaia The Fierce Knight", "Swift Gaia the Fierce Knight", or "Gaia the Dragon Champion" you control attacks a Defense Position monster, inflict piercing battle damage to your opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.pietg)
    effect::set_target(d, e1, Some(effect_card_fn(pietg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --If "Gaia the Dragon Champion" inflicts battle damage with this effect: Draw 2 cards, then discard 1 card.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.pietg(e,c)`
fn pietg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsCode(6368038,16589042,CARD_GAIA_CHAMPION)
    card::is_code(d, c, &[6368038, 16589042, CARD_GAIA_CHAMPION])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc);
    // return tc:IsControler(tp) and bc and bc:IsDefensePos() and tc:IsCode(CARD_GAIA_CHAMPION)
    card::is_controler(d, tc, a.tp)
        && bc.is_some_and(|bc| card::is_defense_pos(d, bc))
        && card::is_code(d, tc, &[CARD_GAIA_CHAMPION])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.Draw(tp,2,REASON_EFFECT)==2 then
    if duel::draw(d, tp, 2, REASON_EFFECT) == 2 {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
