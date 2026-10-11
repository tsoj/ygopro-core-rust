//! Sunvine Gardna
use super::prelude::*;

pub const ID: u32 = 28168762;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUNAVALON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PLANT, sc, st, Some(tp)) && card::is_type_ctx(d, c, TYPE_NORMAL, sc, st, Some(tp)))),
        1,
        Some(1),
        None,
        None,
        None,
    );
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // halve damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.dmgcon)
    effect::set_condition(d, e2, Some(condition(dmgcon)));
    // e2:SetOperation(s.dmgop)
    effect::set_operation(d, e2, Some(operation(dmgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // bp end
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op3)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,tp,rp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, rp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsSetCard(SET_SUNAVALON) and c:IsType(TYPE_LINK)
    //   and c:GetPreviousControler()==tp and c:IsReason(REASON_EFFECT)
    let _ = rp;
    card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_set_card(d, c, &[SET_SUNAVALON as u16])
        && card::is_type(d, c, TYPE_LINK)
        && card::get_previous_controler(d, c) == tp
        && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp,rp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp, a.rp), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
/// `s.dmgfilter(c,cc)`
fn dmgfilter(d: &mut Duel, c: CardId, cc: CardId) -> bool {
    // return c:IsSetCard(SET_SUNAVALON) and c:IsType(TYPE_LINK) and c:GetLinkedGroup():IsContains(cc)
    card::is_set_card(d, c, &[SET_SUNAVALON as u16]) && card::is_type(d, c, TYPE_LINK) && group::is_contains(&card::get_linked_group(d, c), cc)
}
/// `s.dmgcon(e,tp,eg,ep,ev,re,r,rp)`
fn dmgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsRelateToBattle() and c:GetBattleTarget()
    //   and Duel.IsExistingMatchingCard(s.dmgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,c)
    card::is_faceup(d, c)
        && card::is_relate_to_battle(d, c)
        && card::get_battle_target(d, c).is_some()
        && duel::is_existing_matching_card(d, Some(&|d, x| dmgfilter(d, x, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.dmgop(e,tp,eg,ep,ev,re,r,rp)`
fn dmgop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(HALF_DAMAGE)
    effect::set_value(d, e1, HALF_DAMAGE as i64);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)` (the operation of the third effect)
fn op3(d: &mut Duel, a: &EffArgs) {
    // local e2=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_ADJUST)
    effect::set_code(d, e2, EVENT_ADJUST);
    // e2:SetOperation(s.skop)
    effect::set_operation(d, e2, Some(operation(skop)));
    // e2:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.skop(e,tp,eg,ep,ev,re,r,rp)`
fn skop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetCurrentPhase()~=PHASE_DAMAGE then
    if duel::get_current_phase(d) != PHASE_DAMAGE {
        // local p=Duel.GetTurnPlayer()
        let p = duel::get_turn_player(d);
        // Duel.SkipPhase(p,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        duel::skip_phase(d, p, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
        // e:Reset()
        effect::reset(d, a.e);
    }
}
