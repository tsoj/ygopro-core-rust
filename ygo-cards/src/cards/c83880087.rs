//! Light Wing Shield
use super::prelude::*;

pub const ID: u32 = 83880087;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_DISABLED)
    effect::set_code(d, e1, EVENT_ATTACK_DISABLED);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach cost change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e2, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.rcon)
    effect::set_condition(d, e2, Some(condition(rcon)));
    // e2:SetOperation(s.rop)
    effect::set_operation(d, e2, Some(operation_count(rop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_UTOPIA)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_UTOPIA as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=0
    // if Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) then
    //     op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    // else
    //     op=Duel.SelectOption(tp,aux.Stringid(id,0))
    // end
    let op = if duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0)])
    };
    if op == 0 {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // e:SetOperation(s.endop)
        effect::set_operation(d, e, Some(operation(endop)));
    } else {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, e, CATEGORY_ATKCHANGE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
        let _g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
        // e:SetOperation(s.atkop)
        effect::set_operation(d, e, Some(operation(atkop)));
    }
    true
}
/// `s.endop(e,tp,eg,ep,ev,re,r,rp)`
fn endop(d: &mut Duel, a: &EffArgs) {
    // local turnp=Duel.GetTurnPlayer()
    let turnp = duel::get_turn_player(d);
    // Duel.SkipPhase(turnp,PHASE_MAIN1,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, turnp, PHASE_MAIN1, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(turnp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1,1)
    duel::skip_phase(d, turnp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
    // Duel.SkipPhase(turnp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, turnp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,turnp)
    duel::register_effect(d, e1, turnp);
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetBaseAttack()*2)
        let base = card::get_base_attack(d, tc);
        effect::set_value(d, e1, base as i64 * 2);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.rcon(e,tp,eg,ep,ev,re,r,rp)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // return (r&REASON_COST)~=0 and re:IsActivated()
    //     and re:IsActiveType(TYPE_XYZ) and re:GetHandler():IsSetCard(SET_UTOPIA)
    //     and e:GetHandler():IsAbleToRemoveAsCost()
    //     and ep==e:GetOwnerPlayer() and ev==1
    if (a.r & REASON_COST) == 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_activated(d, re) || !effect::is_active_type(d, re, TYPE_XYZ) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    if !card::is_set_card(d, rc, &[SET_UTOPIA as u16]) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    card::is_able_to_remove_as_cost(d, h, None) && a.ep == effect::get_owner_player(d, e) && a.ev == 1
}
/// `s.rop(e,tp,eg,ep,ev,re,r,rp)`
fn rop(d: &mut Duel, a: &EffArgs) -> i64 {
    // return Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_COST, None, None) as i64
}
