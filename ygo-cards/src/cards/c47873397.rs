//! Totem Pole
use super::prelude::*;

pub const ID: u32 = 47873397;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_SELF_TOGRAVE)
    duel::enable_global_flag(d, GLOBALFLAG_SELF_TOGRAVE);
    // c:EnableCounterPermit(0x20f)
    card::enable_counter_permit(d, c, 0x20f, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Opponent cannot target monsters with 0 original ATK with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(s.filter))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| filter(d, c))));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(s.btcost)
    effect::set_cost(d, e2, Some(cost_target(btcost)));
    // e2:SetOperation(s.btop)
    effect::set_operation(d, e2, Some(operation(btop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send this card to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e3, EFFECT_SELF_TOGRAVE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(function(e) return e:GetHandler():GetCounter(0x20f)==3 end)
    effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Double any effect damage the opponent takes this turn
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCondition(s.damcon)
    effect::set_condition(d, e4, Some(condition(damcon)));
    // e4:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e4, Some(cost::self_banish()));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ROCK) and c:GetBaseAttack()==0
    card::is_race(d, c, RACE_ROCK as u64) && card::get_base_attack(d, c) == 0
}
/// `s.btcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn btcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:HasFlagEffect(id) and c:IsCanAddCounter(0x20f,1) end
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None) && card::is_can_add_counter(d, c, 0x20f, 1, false, 0);
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    true
}
/// `s.btop(e,tp,eg,ep,ev,re,r,rp)`
fn btop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.NegateAttack() and c:IsRelateToEffect(e) and c:IsCanAddCounter(0x20f,1) then
    if duel::negate_attack(d) && card::is_relate_to_effect(d, c, a.e) && card::is_can_add_counter(d, c, 0x20f, 1, false, 0) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:AddCounter(0x20f,1)
        card::add_counter(d, c, 0x20f, 1, false);
    }
}
/// `function(_,_,_,_) ... (e:GetHandler():GetCounter(0x20f)==3)` (e3 condition)
fn e3_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_counter(d, h, 0x20f) == 3
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.HasFlagEffect(tp,id) and Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetCode)>=3
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return false;
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(function(_,_,val,r) return r&REASON_EFFECT>0 and val*2 or val end)
    effect::set_value_fn(d, e1, lua_fn(damage_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `function(_,_,val,r) return r&REASON_EFFECT>0 and val*2 or val end`
fn damage_value(_d: &mut Duel, a: Args) -> i64 {
    let val = a.int(2);
    let r = a.int(3);
    if (r & REASON_EFFECT as i64) > 0 { val * 2 } else { val }
}
