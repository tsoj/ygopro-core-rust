//! Soul Devouring Bamboo Sword
use super::prelude::*;

pub const ID: u32 = 51670553;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BAMBOO_SWORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // skip draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetCondition(s.skipcon)
    effect::set_condition(d, e2, Some(condition(skipcon)));
    // e2:SetOperation(s.skipop)
    effect::set_operation(d, e2, Some(operation(skipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetEquipCount()~=0 and c:GetEquipGroup():IsExists(Card.IsSetCard,1,nil,SET_BAMBOO_SWORD)
    if card::get_equip_count(d, c) == 0 {
        return false;
    }
    let eqg = card::get_equip_group(d, c);
    group::is_exists(d, &eqg, &|d, x| card::is_set_card(d, x, &[SET_BAMBOO_SWORD as u16]), 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // e:GetHandler():SetTurnCounter(0)
    let h = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, h, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
    }
}
// function s.skipcon(e,tp,eg,ep,ev,re,r,rp)
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let Some(tc) = card::get_first_card_target(d, h) else {
        // return tc and ...
        return false;
    };
    // return tc and ep~=tp and r==REASON_BATTLE and (Duel.GetAttacker()==tc or Duel.GetAttackTarget()==tc)
    a.ep != a.tp && a.r == REASON_BATTLE && (duel::get_attacker(d) == Some(tc) || duel::get_attack_target(d) == Some(tc))
}
// function s.skipop(e,tp,eg,ep,ev,re,r,rp)
fn skipop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCode(EFFECT_SKIP_DP)
    effect::set_code(d, e1, EFFECT_SKIP_DP);
    // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_OPPO_TURN)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_OPPO_TURN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()
    let ct = card::get_turn_counter(d, c);
    // ct=ct+1
    let ct = ct + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
    // if ct==2 then
    if ct == 2 {
        // Duel.Destroy(c,REASON_RULE)
        duel::destroy(d, CardSet::from_card(c), REASON_RULE, None, None);
    }
}
