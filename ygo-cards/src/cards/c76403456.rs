//! Power Wall
use super::prelude::*;

pub const ID: u32 = 76403456;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp)
    let atk = duel::get_attacker(d).nil();
    card::is_controler(d, atk, 1 - a.tp)
}
/// `math.ceil(Duel.GetBattleDamage(tp)/500)`
fn val_of(d: &Duel, tp: u8) -> i64 {
    (duel::get_battle_damage(d, tp) as f64 / 500.0).ceil() as i64
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local val=math.ceil(Duel.GetBattleDamage(tp)/500)
    let val = val_of(d, tp);
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,val)
    //   and not Duel.IsPlayerAffectedByEffect(tp,EFFECT_AVOID_BATTLE_DAMAGE) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, tp, val as u32) && duel::is_player_affected_by_effect(d, tp, EFFECT_AVOID_BATTLE_DAMAGE).is_none();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local val=math.ceil(Duel.GetBattleDamage(tp)/500)
    let val = val_of(d, tp);
    // Duel.DiscardDeck(tp,val,REASON_EFFECT)
    duel::discard_deck(d, tp, val as u16, REASON_EFFECT);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if og:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)<val then return end
    let n = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) as i64;
    if n < val {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
