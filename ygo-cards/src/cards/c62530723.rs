//! World Legacy Whispers
use super::prelude::*;

pub const ID: u32 = 62530723;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKK_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Negate Spell effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(5)
    card::is_faceup(d, c) && card::is_level_above(d, c, 5)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && atkfilter(d, chkc);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.IsPhase(PHASE_DAMAGE) then
        if duel::is_phase(d, PHASE_DAMAGE) {
            // return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
            return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
        }
        // return true
        return true;
    }
    // if Duel.IsPhase(PHASE_DAMAGE)
    //   or (Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,0))) then
    if duel::is_phase(d, PHASE_DAMAGE)
        || (duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)))
    {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // e:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetOperation(s.activate)
        effect::set_operation(d, a.e, Some(operation(activate)));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
        effect::set_property(d, a.e, EFFECT_FLAG_DAMAGE_STEP, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(1000)
            effect::set_value(d, e1, 1000);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `s.cfilter(c,seq,p)`
fn cfilter(d: &mut Duel, c: CardId, seq: u32, p: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MEKK_KNIGHT) and c:IsColumn(seq,p,LOCATION_SZONE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_column(d, c, seq, Some(p), Some(LOCATION_SZONE))
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not re:IsSpellEffect() then return false end
    if a.rp == a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_spell_effect(d, re) {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // local p,loc,seq=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_SEQUENCE)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SEQUENCE]);
    let p = info[0].int();
    let mut loc = info[1].int();
    let mut seq = info[2].int();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and (loc&LOCATION_SZONE==0 or rc:IsControler(1-p)) then
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && ((loc & LOCATION_SZONE as i64) == 0 || card::is_controler(d, rc, (1 - p) as u8))
    {
        // if rc:IsLocation(LOCATION_SZONE) and rc:IsControler(p) then
        if card::is_location(d, rc, LOCATION_SZONE) && card::is_controler(d, rc, p as u8) {
            // seq=rc:GetSequence()
            seq = card::get_sequence(d, rc) as i64;
            // loc=LOCATION_SZONE
            loc = LOCATION_SZONE as i64;
        } else {
            // seq=rc:GetPreviousSequence()
            seq = card::get_previous_sequence(d, rc) as i64;
            // loc=rc:GetPreviousLocation()
            loc = card::get_previous_location(d, rc) as i64;
        }
    }
    // return loc&LOCATION_SZONE==LOCATION_SZONE and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,seq,p)
    (loc & LOCATION_SZONE as i64) == LOCATION_SZONE as i64
        && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, seq as u32, p as u8)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
