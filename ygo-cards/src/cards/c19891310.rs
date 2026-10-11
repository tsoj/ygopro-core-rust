//! Geargiagear Gigant XG
use super::prelude::*;

pub const ID: u32 = 19891310;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEARGIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,3,3)
    xyz::add_procedure(d, c, None, Some(3), 3, None, None, None, None, false, None);
    // Until the end of the Damage Step, negate the effects ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(TIMING_ATTACK|TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e1, TIMING_ATTACK | TIMING_BATTLE_PHASE, None);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(1),Cost.SoftOncePerBattle))
    // Cost.SoftOncePerBattle is used without arguments: it always passes and records nothing (see c56832966.rs).
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_no_op()])));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Add 1 other "Geargia" card from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousPosition(POS_FACEUP) end)
    effect::set_condition(d, e2, Some(condition(leave_cond)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn cost_no_op() -> LuaFn {
    cost_target(|_d, _a| true)
}
// s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    // return bc and bc:IsRace(RACE_MACHINE) and bc:IsFaceup()
    match bc {
        Some(bc) => card::is_race(d, bc, RACE_MACHINE) && card::is_faceup(d, bc),
        None => false,
    }
}
// s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable(d, x)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // for nc in g:Iter() do
    for nc in group::iter(d, &g) {
        // nc:NegateEffects(c,RESET_PHASE|PHASE_DAMAGE,true)
        card::negate_effects(d, nc, c, Some(RESET_PHASE | PHASE_DAMAGE), true, None);
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GEARGIA) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GEARGIA as u16]) && card::is_able_to_hand(d, c, None)
}
// function(e) return e:GetHandler():IsPreviousPosition(POS_FACEUP) end
fn leave_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_position(d, c, POS_FACEUP as u32)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc~=c and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && chkc != c && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| thfilter(d, x)), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_target(d, a.tp, Some(&|d, x| thfilter(d, x)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
