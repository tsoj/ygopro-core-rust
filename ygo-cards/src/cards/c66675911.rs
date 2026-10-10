//! Nehshaddoll Genius
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 66675911;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted "Shaddoll" monster becomes unaffected by monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.immtg)
    effect::set_target(d, e1, Some(cost_target(immtg)));
    // e1:SetOperation(s.immop)
    effect::set_operation(d, e1, Some(operation(immop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Targeted monster cannot activate its effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.immfilter(c)`
fn immfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_faceup(d, c)
}
/// `s.immtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn immtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.immfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && immfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.immfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&immfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.immfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&immfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.immop(e,tp,eg,ep,ev,re,r,rp)`
fn immop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3101)
            effect::set_description(d, e1, 3101);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
            // e1:SetValue(s.efilter)
            effect::set_value_fn(d, e1, lua_fn(efilter));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsMonsterEffect() and te:GetOwner()~=e:GetHandler()
    let e = a.e(0);
    let te = a.e(1);
    let h = effect::get_handler(d, e).nil();
    effect::is_monster_effect(d, te) && effect::get_owner(d, te) != Some(h)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`: "If sent to the GY by a card effect"
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsType(TYPE_EFFECT) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_type(d, chkc, TYPE_EFFECT);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    let f = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EFFECT));
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&f), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&f), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`: "Targeted monster cannot activate its effects"
fn e2_operation(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsType(TYPE_EFFECT) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_type(d, tc, TYPE_EFFECT) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3302)
            effect::set_description(d, e1, 3302);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
    }
}
