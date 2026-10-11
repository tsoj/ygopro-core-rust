//! Abyss Actor - Funky Comedian
use super::prelude::*;

pub const ID: u32 = 99634927;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ABYSS_ACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Enable pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // ATK up (pendulum zone)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.atkcost1)
    effect::set_cost(d, e1, Some(cost_target(atkcost1)));
    // e1:SetTarget(s.atktg1)
    effect::set_target(d, e1, Some(cost_target(atktg1)));
    // e1:SetOperation(s.atkop1)
    effect::set_operation(d, e1, Some(operation(atkop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ATK up (if normal or special summoned)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.atktg2)
    effect::set_target(d, e2, Some(cost_target(atktg2)));
    // e2:SetOperation(s.atkop2)
    effect::set_operation(d, e2, Some(operation(atkop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // ATK up (ignition effect)
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCost(s.atkcost3)
    effect::set_cost(d, e4, Some(cost_target(atkcost3)));
    // e4:SetTarget(s.atktg3)
    effect::set_target(d, e4, Some(cost_target(atktg3)));
    // e4:SetOperation(s.atkop3)
    effect::set_operation(d, e4, Some(operation(atkop3)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkfilter1(c,tp)`
fn atkfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ABYSS_ACTOR) and Duel.IsExistingTarget(s.atkfilter2,tp,LOCATION_MZONE,0,1,c)
    card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && duel::is_existing_target(d, Some(&atkfilter2), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.atkfilter2(c)`
fn atkfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ABYSS_ACTOR) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && card::is_faceup(d, c)
}
/// `s.atkcost1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.atkfilter1,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| atkfilter1(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.atkfilter1,1,1,false,nil,nil,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| atkfilter1(d, c, tp)), 1, 1, false, None, NO_EX);
    // e:SetLabel(g:GetFirst():GetBaseAttack())
    let first = group::get_first(d, &g).nil();
    let atk = card::get_base_attack(d, first);
    effect::set_label(d, e, &[atk as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, &g, REASON_COST, None);
    true
}
/// `s.atktg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter2(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && atkfilter2(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter2,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter2), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.atkfilter2,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&atkfilter2), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop1(e,tp,eg,ep,ev,re,r,rp)`
fn atkop1(d: &mut Duel, a: &EffArgs) {
    let (e, _tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        let tc = tc.nil();
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(e:GetLabel())
            effect::set_value(d, e1, effect::get_label(d, e));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.atktg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkfilter2,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkfilter2), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.atkop2(e,tp,eg,ep,ev,re,r,rp)`
fn atkop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) {
        // local atkval=Duel.GetMatchingGroupCount(s.atkfilter2,tp,LOCATION_MZONE,0,nil)*300
        let atkval = duel::get_matching_group_count(d, Some(&atkfilter2), tp, LOCATION_MZONE, 0, NO_EX) * 300;
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atkval)
        effect::set_value(d, e1, atkval as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.atkcost3(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost3(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, c) == 0;
    }
    // Cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    true
}
/// `s.atktg3(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg3(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc~=c and chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter2(chkc) end
    if let Some(chkc) = a.chkc() {
        return chkc != c && card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && atkfilter2(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter2,tp,LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter2), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
    // Duel.SelectTarget(tp,s.atkfilter2,tp,LOCATION_MZONE,0,1,1,c)
    let _ = duel::select_target(d, tp, Some(&atkfilter2), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.atkop3(e,tp,eg,ep,ev,re,r,rp)`
fn atkop3(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
        // local atk=c:GetAttack()
        let atk = card::get_attack(d, c);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
