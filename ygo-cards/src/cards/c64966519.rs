//! Dispatchparazzi
use super::prelude::*;

pub const ID: u32 = 64966519;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // change target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.cbcon)
    effect::set_condition(d, e1, Some(condition(cbcon)));
    // e1:SetOperation(s.cbop)
    effect::set_operation(d, e1, Some(operation(cbop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cbcon(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bt=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let bt = group::get_first(d, &eg);
    // return r~=REASON_REPLACE and bt~=e:GetHandler() and bt:IsControler(tp)
    if a.r == REASON_REPLACE {
        return false;
    }
    let h = effect::get_handler(d, a.e);
    if bt == h {
        return false;
    }
    card::is_controler(d, bt.nil(), a.tp)
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local at=Duel.GetAttacker()
        let at = duel::get_attacker(d).nil();
        // if at:CanAttack() and not at:IsImmuneToEffect(e) and not c:IsImmuneToEffect(e) then
        if card::can_attack(d, at) && !card::is_immune_to_effect(d, at, a.e) && !card::is_immune_to_effect(d, c, a.e) {
            // Duel.CalculateDamage(at,c)
            duel::calculate_damage(d, at, Some(c), false);
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsReason(REASON_BATTLE) and c:GetBattleTarget():IsControler(1-tp) and rp==1-tp
    card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_BATTLE)
        && card::is_controler(d, card::get_battle_target(d, c).nil(), 1 - a.tp)
        && a.rp == 1 - a.tp
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, c);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,tc:GetBaseAttack()/2)
    let base = card::get_base_attack(d, tc.nil());
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, lua_int(base as f64 / 2.0) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, c).nil();
    // if Duel.Destroy(tc,REASON_EFFECT)==0 then return end
    if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Recover(tp,tc:GetBaseAttack()/2,REASON_EFFECT)
    let base = card::get_base_attack(d, tc);
    duel::recover(d, a.tp, lua_int(base as f64 / 2.0), REASON_EFFECT, false, None);
}
