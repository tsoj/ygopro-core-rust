//! Springans Booty
use super::prelude::*;

pub const ID: u32 = 7496001;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[60884672], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Make 1 of opponent's monsters unable to activate its effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(s.atrfilter,1,nil,tp,rp) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.catg)
    effect::set_target(d, e1, Some(cost_target(catg)));
    // e1:SetOperation(s.caop)
    effect::set_operation(d, e1, Some(operation(caop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 "Great Sea Sand - Gold Golgonda" from deck or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetTarget(s.actg)
    effect::set_target(d, e2, Some(cost_target(actg)));
    // e2:SetOperation(s.acop)
    effect::set_operation(d, e2, Some(operation(acop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atrfilter(c,tp,rp)
fn atrfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsType(TYPE_XYZ) and c:IsPreviousControler(tp) and c:IsReason(REASON_EFFECT)
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
}
// e1 condition: function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(s.atrfilter,1,nil,tp,rp) end
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| atrfilter(d, c, a.tp), 1, NO_EX)
}
// function s.catg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn catg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsType(TYPE_EFFECT) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_faceup(d, chkc)
            && card::is_type(d, chkc, TYPE_EFFECT);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,1,1,nil)
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.caop(e,tp,eg,ep,ev,re,r,rp)
fn caop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        let h = effect::get_handler(d, a.e).nil();
        // Cannot activate its effects
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3302)
        effect::set_description(d, e1, 3302);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
}
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(60884672) and c:GetActivateEffect() and c:GetActivateEffect():IsActivatable(tp,true,true)
    //     and (c:IsType(TYPE_FIELD) or Duel.GetLocationCount(tp,LOCATION_SZONE)>0)
    if !card::is_code(d, c, &[60884672]) {
        return false;
    }
    let Some(ae) = card::get_activate_effect(d, c).first().copied() else {
        return false;
    };
    effect::is_activatable(d, ae, tp, true, true)
        && (card::is_type(d, c, TYPE_FIELD) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
}
// function s.actg(e,tp,eg,ep,ev,re,r,rp,chk)
fn actg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
// function s.acop(e,tp,eg,ep,ev,re,r,rp)
fn acop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| filter(d, c, a.tp))),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &sel);
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // if tc:IsFieldSpell() then
    if card::is_field_spell(d, tc) {
        // Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp)
        duel::activate_field_spell(d, Some(tc), a, Some(a.tp));
    } else {
        // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        // local te=tc:GetActivateEffect()
        let te = card::get_activate_effect(d, tc).first().copied().nil();
        // local tep=tc:GetControler()
        let tep = card::get_controler(d, tc);
        // local cost=te:GetCost()
        // if cost then cost(te,tep,eg,ep,ev,re,r,rp,1) end
        if let Some(cost) = effect::get_cost(d, te) {
            let args = [
                Value::Effect(te),
                Value::Int(tep as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(1),
            ];
            cost.call(d, &args);
        }
    }
}
