//! Vaylantz Dominator Duke
use super::prelude::*;

pub const ID: u32 = 13291886;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_VALIANTS_KOENIGWISSEN],
    listed_series: &[SET_VAYLANTZ],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Special Summon self
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent activation
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.catg)
    effect::set_target(d, e2, Some(cost_target(catg)));
    // e2:SetOperation(s.caop)
    effect::set_operation(d, e2, Some(operation(caop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Take control
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e3, CATEGORY_CONTROL);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_MOVE)
    effect::set_code(d, e3, EVENT_MOVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.ctcon)
    effect::set_condition(d, e3, Some(condition(ctcon)));
    // e3:SetTarget(s.cttg)
    effect::set_target(d, e3, Some(cost_target(cttg)));
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.spconfilter(c)
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAYLANTZ) and c:IsAttribute(ATTRIBUTE_FIRE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::is_attribute(d, c, ATTRIBUTE_FIRE)
}
// s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_VALIANTS_KOENIGWISSEN,PLAYER_ALL,LOCATION_FZONE)
    //   or Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_environment(d, CARD_VALIANTS_KOENIGWISSEN, Some(PLAYER_ALL as u8), Some(LOCATION_FZONE))
        || duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // return zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
        return zone != 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local zone=(1<<c:GetSequence())&ZONES_MMZ
    let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
    // if zone~=0 then
    if zone != 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
// s.cafilter(c)
fn cafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()<5
    card::is_facedown(d, c) && card::get_sequence(d, c) < 5
}
// s.catg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn catg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.cafilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && cafilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cafilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cafilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWN)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEDOWN as u64);
    // Duel.SelectTarget(tp,s.cafilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&cafilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX);
    true
}
// s.caop(e,tp,eg,ep,ev,re,r,rp)
fn caop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // --Cannot activate card this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// s.ctcon(e,tp,eg,ep,ev,re,r,rp)
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
// s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsControlerCanBeChanged()
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
// s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MMZONE) and chkc:IsControler(1-tp) and s.ctfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MMZONE) && card::is_controler(d, chkc, 1 - a.tp) && ctfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ctfilter,tp,0,LOCATION_MMZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ctfilter), a.tp, 0, LOCATION_MMZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.ctfilter,tp,0,LOCATION_MMZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&ctfilter), a.tp, 0, LOCATION_MMZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    true
}
// s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)
    //   and tc:IsControler(1-tp) and tc:IsControlerCanBeChanged() and Duel.GetControl(tc,tp) then
    if card::is_faceup(d, tc)
        && card::is_relate_to_effect(d, tc, a.e)
        && !card::is_immune_to_effect(d, tc, a.e)
        && card::is_controler(d, tc, 1 - tp)
        && card::is_controler_can_be_changed(d, tc, false, None)
        && { duel::get_control(d, tc, tp, 0, 0, None, None); true } /* (a Lua number is always true) */
    {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --Cannot activate its effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3302)
        effect::set_description(d, e1, 3302);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // --Cannot declare an attack
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetDescription(3206)
        effect::set_description(d, e2, 3206);
        // e2:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // --Treated as a "Vaylantz" monster
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e3, aux::stringid(ID, 3));
        // e3:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_ADD_SETCODE)
        effect::set_code(d, e3, EFFECT_ADD_SETCODE);
        // e3:SetValue(SET_VAYLANTZ)
        effect::set_value(d, e3, SET_VAYLANTZ as i64);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
