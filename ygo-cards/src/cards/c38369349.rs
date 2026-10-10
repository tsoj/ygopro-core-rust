//! Manga Ryu-Ran
use super::prelude::*;

pub const ID: u32 = 38369349;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15259703], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(s.sdescon)
    effect::set_condition(d, e2, Some(condition(sdescon)));
    // e2:SetOperation(s.sdesop)
    effect::set_operation(d, e2, Some(operation(sdesop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Direct attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // e3:SetCondition(s.dircon)
    effect::set_condition(d, e3, Some(condition(dircon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e4:SetCondition(s.atcon)
    effect::set_condition(d, e4, Some(condition(atcon)));
    // e4:SetValue(s.atlimit)
    effect::set_value_fn(d, e4, effect_card_fn(atlimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e5, EFFECT_CANNOT_DIRECT_ATTACK);
    // e5:SetCondition(s.atcon)
    effect::set_condition(d, e5, Some(condition(atcon)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Cannot attack
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e6:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e6:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e6, EVENT_SPSUMMON_SUCCESS);
    // e6:SetOperation(s.atklimit)
    effect::set_operation(d, e6, Some(operation(atklimit)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Attack cost
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetCode(EFFECT_ATTACK_COST)
    effect::set_code(d, e7, EFFECT_ATTACK_COST);
    // e7:SetCost(s.atcost)
    effect::set_cost(d, e7, Some(lua_fn(atcost)));
    // e7:SetOperation(s.atop)
    effect::set_operation(d, e7, Some(operation(atop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `function(e,c)` of `e1:SetCondition` (the Special Summon procedure)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,aux.TRUE,2,false,2,true,c,tp,nil,false,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),tp,LOCATION_ONFIELD,0,1,nil)
    duel::check_release_group(
        d,
        tp,
        None,
        2,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(2), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    ) && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[15259703]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c)` of `e1:SetTarget` (the Special Summon procedure)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(8);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
    // local g=Duel.SelectReleaseGroup(tp,aux.TRUE,2,2,false,true,true,c,nil,nil,false,nil)
    let g = duel::select_release_group(
        d,
        tp,
        None,
        2,
        2,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: Some(c), ..Default::default() },
        NO_EX,
        false,
    );
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c)` of `e1:SetOperation` (the Special Summon procedure)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // g:DeleteGroup()
    }
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
