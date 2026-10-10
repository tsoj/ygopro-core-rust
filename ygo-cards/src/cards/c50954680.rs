//! Crystal Wing Synchro Dragon
use super::prelude::*;

pub const ID: u32 = 50954680;

pub const INFO: ScriptInfo = ScriptInfo { synchro_nt_required: Some(1), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsType,TYPE_SYNCHRO),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // return re:IsMonsterEffect() and rc~=c and not c:IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    effect::is_monster_effect(d, re) && rc != c && !card::is_status(d, c.nil(), STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) and Duel.Destroy(rc,REASON_EFFECT)~=0
    //   and rc:GetBaseAttack()>=0 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if duel::negate_activation(d, a.ev as u8)
        && card::is_relate_to_effect(d, rc.nil(), re)
        && duel::destroy(d, CardSet::from_card(rc.nil()), REASON_EFFECT, None, None) != 0
        && card::get_base_attack(d, rc.nil()) >= 0
        && card::is_relate_to_effect(d, c.nil(), a.e)
        && card::is_faceup(d, c.nil())
    {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c.nil());
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // e1:SetValue(rc:GetBaseAttack())
        let atk = card::get_base_attack(d, rc.nil());
        effect::set_value(d, e1, atk as i64);
        // c:RegisterEffect(e1)
        card::register_effect(d, c.nil(), e1, false);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsLevelAbove(5) and bc:IsControler(1-tp)
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_level_above(d, bc, 5) && card::is_controler(d, bc, 1 - a.tp),
        None => false,
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if c:IsRelateToBattle() and c:IsFaceup() and bc:IsRelateToBattle() and bc:IsFaceup() then
    if card::is_relate_to_battle(d, c) && card::is_faceup(d, c) && card::is_relate_to_battle(d, bc.nil()) && card::is_faceup(d, bc.nil()) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // e1:SetValue(bc:GetAttack())
        let atk = card::get_attack(d, bc.nil());
        effect::set_value(d, e1, atk as i64);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
