//! Dharma-Eye Magician
use super::prelude::*;

pub const ID: u32 = 88757791;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum attributes
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Change scale
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.sctg)
    effect::set_target(d, e2, Some(cost_target(sctg)));
    // e2:SetOperation(s.scop)
    effect::set_operation(d, e2, Some(operation(scop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register when pendulum summoned
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetOperation(s.sumsuc)
    effect::set_operation(d, e3, Some(operation(sumsuc)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Prevent destruction by opponent's effect
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetCondition(s.indcon)
    effect::set_condition(d, e4, Some(condition(indcon)));
    // e4:SetTarget(s.indtg)
    effect::set_target(d, e4, Some(effect_card_fn(indtg)));
    // e4:SetValue(aux.indoval)
    effect::set_value_fn(d, e4, aux::indoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_PENDULUM) and not c:IsPublic() and Duel.IsExistingTarget(s.scfilter,tp,LOCATION_PZONE,0,1,nil,c)
    card::is_type(d, c, TYPE_PENDULUM)
        && !card::is_public(d, c)
        && duel::is_existing_target(d, Some(&|d, x| scfilter(d, x, c)), tp, LOCATION_PZONE, 0, 1, NO_EX)
}
// function s.scfilter(c,pc)
fn scfilter(d: &mut Duel, c: CardId, pc: CardId) -> bool {
    // return c:IsSetCard(SET_MAGICIAN) and c:GetLeftScale()~=pc:GetLeftScale()
    card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::get_left_scale(d, c) != card::get_left_scale(d, pc)
}
// function s.sctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_PZONE) and s.scfilter(chkc,e:GetLabelObject()) end
    if let Some(chkc) = a.chkc() {
        let pc = effect::get_label_object(d, a.e).as_card().nil();
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_PZONE) && scfilter(d, chkc, pc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let tp = a.tp;
    let cg = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - a.tp, cg.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabelObject(cg:GetFirst())
    let first = group::get_first(d, &cg).nil();
    effect::set_label_object(d, a.e, Value::Card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.scfilter,tp,LOCATION_PZONE,0,1,1,nil,cg:GetFirst())
    duel::select_target(d, a.tp, Some(&|d, x| scfilter(d, x, first)), a.tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.scop(e,tp,eg,ep,ev,re,r,rp)
fn scop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local pc=e:GetLabelObject()
    let pc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let pc = pc.nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LSCALE)
            effect::set_code(d, e1, EFFECT_CHANGE_LSCALE);
            // e1:SetValue(pc:GetLeftScale())
            let lscale = card::get_left_scale(d, pc) as i64;
            effect::set_value(d, e1, lscale);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_RSCALE)
            effect::set_code(d, e2, EFFECT_CHANGE_RSCALE);
            // e2:SetValue(pc:GetRightScale())
            let rscale = card::get_right_scale(d, pc) as i64;
            effect::set_value(d, e2, rscale);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
// function s.sumsuc(e,tp,eg,ep,ev,re,r,rp)
fn sumsuc(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_PHASE_END&~(RESET_TEMP_REMOVE|RESET_TURN_SET),0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !(RESET_TEMP_REMOVE | RESET_TURN_SET)), 0, 1, 0, 0);
}
// function s.indcon(e)
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetFlagEffect(id)~=0 and c:IsPendulumSummoned()
    card::get_flag_effect(d, c, ID) != 0 && card::is_pendulum_summoned(d, c)
}
// function s.indtg(e,c)
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGICIAN) and c:IsType(TYPE_PENDULUM)
    card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::is_type(d, c, TYPE_PENDULUM)
}
