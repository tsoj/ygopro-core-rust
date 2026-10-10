//! Number 9: Dyson Sphere
use super::prelude::*;

pub const ID: u32 = 1992816;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(9), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,9,2)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // negate attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_PHASE));
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCondition(s.olcon)
    effect::set_condition(d, e2, Some(condition(olcon)));
    // e2:SetTarget(s.oltg)
    effect::set_target(d, e2, Some(cost_target(oltg)));
    // e2:SetOperation(s.olop)
    effect::set_operation(d, e2, Some(operation(olop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // direct attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.dacon)
    effect::set_condition(d, e3, Some(condition(dacon)));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e3:SetOperation(s.daop)
    effect::set_operation(d, e3, Some(operation(daop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    // if at and at:IsStatus(STATUS_ATTACK_CANCELED) then return false end
    if let Some(at) = duel::get_attacker(d) {
        if card::is_status(d, at, STATUS_ATTACK_CANCELED) {
            return false;
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c==Duel.GetAttackTarget() and c:GetOverlayCount()>0
    Some(c) == duel::get_attack_target(d) && card::get_overlay_count(d, c) > 0
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0 end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) == 0;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, _a: &EffArgs) {
    // Duel.NegateAttack()
    duel::negate_attack(d);
}
/// `s.olcon(e,tp,eg,ep,ev,re,r,rp)`
fn olcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayCount()==0 and e:GetHandler():IsType(TYPE_XYZ)
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) == 0 && card::is_type(d, h, TYPE_XYZ)
}
/// `s.oltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn oltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and chkc:IsMonster() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && card::is_monster(d, chkc);
    }
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ)
    //   and Duel.IsExistingTarget(Card.IsMonster,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ) && duel::is_existing_target(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsMonster,tp,LOCATION_GRAVE,0,2,2,nil)
    let g = duel::select_target(d, a.tp, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 2, 0, 0);
    true
}
/// `s.olop(e,tp,eg,ep,ev,re,r,rp)`
fn olop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local g=Duel.GetTargetCards(e)
        let g = duel::get_target_cards(d, a.e).nil();
        // if #g>0 then Duel.Overlay(c,g) end
        if !g.is_empty() {
            duel::overlay(d, c, g, false);
        }
    }
}
/// `s.dafilter(c,atk)`
fn dafilter(d: &mut Duel, c: CardId, atk: i64) -> bool {
    // return c:IsFaceup() and c:GetAttack()>atk
    card::is_faceup(d, c) && (card::get_attack(d, c) as i64) > atk
}
/// `s.dacon(e,tp,eg,ep,ev,re,r,rp)`
fn dacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    //   and Duel.IsExistingMatchingCard(s.dafilter,tp,0,LOCATION_MZONE,1,nil,e:GetHandler():GetAttack())
    if !duel::is_phase(d, PHASE_MAIN1) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let atk = card::get_attack(d, h) as i64;
    duel::is_existing_matching_card(d, Some(&|d, c| dafilter(d, c, atk)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.daop(e,tp,eg,ep,ev,re,r,rp)`
fn daop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Direct attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
