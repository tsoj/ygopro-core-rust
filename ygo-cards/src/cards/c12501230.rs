//! Combat Wheel
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 12501230;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x20e)
    card::enable_counter_permit(d, c, 0x20e, Some(LOCATION_MZONE), None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Cannot be destroyed by card effect once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetValue(function(e,_,r,rp) return r&REASON_EFFECT==REASON_EFFECT and rp==1-e:GetHandlerPlayer() end)
    effect::set_value_fn(
        d,
        e1,
        lua_fn(|d, a| {
            let r = a.int(2) as u32;
            (r & REASON_EFFECT) == REASON_EFFECT && a.p(3) == 1 - effect::get_handler_player(d, a.e(0))
        }),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain ATK equal to half the total ATK of other monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_BATTLE_START|TIMING_BATTLE_END|TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_BATTLE_START | TIMING_BATTLE_END | TIMING_DAMAGE_STEP, None);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy all monsters you control
    // local e4a=Effect.CreateEffect(c)
    let e4a = effect::create_effect(d, c);
    // e4a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4a:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4a, EVENT_LEAVE_FIELD_P);
    // e4a:SetOperation(function(e) e:SetLabel(e:GetHandler():GetCounter(0x20e)) end)
    effect::set_operation(d, e4a, Some(operation(e4a_op)));
    // c:RegisterEffect(e4a)
    card::register_effect(d, c, e4a, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYED);
    // e4:SetLabelObject(e4a)
    effect::set_label_object(d, e4, Value::Effect(e4a));
    // e4:SetCondition(function(e) return e:GetLabelObject():GetLabel()>0 end)
    effect::set_condition(d, e4, Some(condition(e4_con)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) e:SetLabel(e:GetHandler():GetCounter(0x20e)) end` (e4a)
fn e4a_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let n = card::get_counter(d, h, 0x20e) as i64;
    effect::set_label(d, a.e, &[n]);
}
/// `function(e) return e:GetLabelObject():GetLabel()>0 end` (e4)
fn e4_con(d: &mut Duel, a: &EffArgs) -> bool {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) > 0
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase()
    //     and aux.StatChangeDamageStepCondition()
    duel::is_turn_player(d, 1 - a.tp) && duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:HasFlagEffect(id) and c:IsCanAddCounter(0x20e,1)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && card::is_can_add_counter(d, c, 0x20e, 1, false, 0)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(
        d,
        a.tp,
        Some(&|d, x| card::is_discardable(d, x, None, None)),
        1,
        1,
        REASON_COST | REASON_DISCARD,
        NO_EX,
    );
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Opponent cannot target other monsters for attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(te,tc) return tc~=te:GetHandler() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, te, tc| tc != effect::get_handler(d, te).nil())));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain ATK
    // local atk=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,c):GetSum(Card.GetAttack)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, 0, c.into());
    let atk = group::get_sum(d, &g, &|d, x| card::get_attack(d, x) as i64);
    // if atk>0 and c:UpdateAttack(atk//2)>0 and c:IsCanAddCounter(0x20e) then
    if atk > 0 && card::update_attack(d, c, atk.div_euclid(2), None, None, None) > 0 && card::is_can_add_counter(d, c, 0x20e, 0, false, 0) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:AddCounter(0x20e,1)
        card::add_counter(d, c, 0x20e, 1, false);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
