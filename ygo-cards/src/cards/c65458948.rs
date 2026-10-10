//! Toon Mermaid
use super::prelude::*;

pub const ID: u32 = 65458948;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15259703], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon rule
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.sdescon)
    effect::set_condition(d, e3, Some(condition(sdescon)));
    // e3:SetOperation(s.sdesop)
    effect::set_operation(d, e3, Some(operation(sdesop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // direct attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e4, EFFECT_DIRECT_ATTACK);
    // e4:SetCondition(s.dircon)
    effect::set_condition(d, e4, Some(condition(dircon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e5, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e5:SetCondition(s.atcon)
    effect::set_condition(d, e5, Some(condition(atcon)));
    // e5:SetValue(s.atlimit)
    effect::set_value_fn(d, e5, effect_card_fn(atlimit));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e6, EFFECT_CANNOT_DIRECT_ATTACK);
    // e6:SetCondition(s.atcon)
    effect::set_condition(d, e6, Some(condition(atcon)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // cannot attack
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e7:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e7, EVENT_SPSUMMON_SUCCESS);
    // e7:SetOperation(s.atklimit)
    effect::set_operation(d, e7, Some(operation(atklimit)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // attack cost
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE);
    // e8:SetCode(EFFECT_ATTACK_COST)
    effect::set_code(d, e8, EFFECT_ATTACK_COST);
    // e8:SetCost(s.atcost)
    effect::set_cost(d, e8, Some(lua_fn(atcost)));
    // e8:SetOperation(s.atop)
    effect::set_operation(d, e8, Some(operation(atop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(15259703)
    card::is_faceup(d, c) && card::is_code(d, c, &[15259703])
}
/// `function(e,c)` of `e2:SetCondition` (the Special Summon procedure)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReason(REASON_DESTROY) and c:IsPreviousPosition(POS_FACEUP) and c:GetPreviousCodeOnField()==15259703
    //     and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::get_previous_code_on_field(d, c).first().copied() == Some(15259703)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.sdescon(e,tp,eg,ep,ev,re,r,rp)`
fn sdescon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.sfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &sfilter, 1, NO_EX)
}
/// `s.sdesop(e,tp,eg,ep,ev,re,r,rp)`
fn sdesop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TOON)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.atkfilter,e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil)
    !duel::is_existing_matching_card(d, Some(&atkfilter), effect::get_handler_player(d, a.e), 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.atkfilter,e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&atkfilter), effect::get_handler_player(d, a.e), 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsType(TYPE_TOON) or c:IsFacedown()
    !card::is_type(d, c, TYPE_TOON) || card::is_facedown(d, c)
}
/// `s.atklimit(e,tp,eg,ep,ev,re,r,rp)`
fn atklimit(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.atcost(e,c,tp)`
fn atcost(d: &mut Duel, a: Args) -> bool {
    // return Duel.CheckLPCost(tp,500)
    duel::check_lp_cost(d, a.p(2), 500)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsAttackCostPaid()~=2 and e:GetHandler():IsLocation(LOCATION_MZONE) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, h, LOCATION_MZONE) {
        // Duel.PayLPCost(tp,500)
        duel::pay_lp_cost(d, a.tp, 500);
        // Duel.AttackCostPaid()
        duel::attack_cost_paid(d, None);
    }
}
