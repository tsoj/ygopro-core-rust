//! Dragoncaller Magician
use super::prelude::*;

pub const ID: u32 = 45667991;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // change race
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTarget(s.rctg1)
    effect::set_target(d, e1, Some(cost_target(rctg1)));
    // e1:SetOperation(s.rcop1)
    effect::set_operation(d, e1, Some(operation(rcop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change race
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.rctg2)
    effect::set_target(d, e2, Some(cost_target(rctg2)));
    // e2:SetOperation(s.rcop2)
    effect::set_operation(d, e2, Some(operation(rcop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // effect gain
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    // e3:SetCondition(s.efcon)
    effect::set_condition(d, e3, Some(condition(efcon)));
    // e3:SetOperation(s.efop)
    effect::set_operation(d, e3, Some(operation(efop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rcfilter(c)`
fn rcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsRace(RACE_DRAGON)
    card::is_faceup(d, c) && !card::is_race(d, c, RACE_DRAGON)
}
/// `s.rctg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rctg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.rcfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && rcfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rcfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rcfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.rcfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&rcfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.rcop1(e,tp,eg,ep,ev,re,r,rp)`
fn rcop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(RACE_DRAGON)
        effect::set_value(d, e1, RACE_DRAGON as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.rctg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rctg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsRace(RACE_DRAGON) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_race(d, h, RACE_DRAGON);
    }
    true
}
/// `s.rcop2(e,tp,eg,ep,ev,re,r,rp)`
fn rcop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(RACE_DRAGON)
        effect::set_value(d, e1, RACE_DRAGON as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.efcon(e,tp,eg,ep,ev,re,r,rp)`
fn efcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_FUSION+REASON_SYNCHRO+REASON_XYZ)~=0 and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    // (Lua: `+` binds tighter than `&`)
    (a.r & (REASON_FUSION + REASON_SYNCHRO + REASON_XYZ)) != 0 && card::is_previous_location(d, effect::get_handler(d, a.e).nil(), LOCATION_ONFIELD)
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(lua_fn(atkcon)));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        // e2:SetValue(TYPE_EFFECT)
        effect::set_value(d, e2, TYPE_EFFECT as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e2,true)
        card::register_effect(d, rc, e2, true);
    }
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    let bc = card::get_battle_target(d, h);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and bc and bc:IsRace(RACE_DRAGON)
    if !(ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) {
        return false;
    }
    match bc {
        Some(bc) => card::is_race(d, bc, RACE_DRAGON),
        None => false,
    }
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetHandler():GetBaseAttack()*2
    let h = effect::get_handler(d, e).nil();
    card::get_base_attack(d, h) as i64 * 2
}
