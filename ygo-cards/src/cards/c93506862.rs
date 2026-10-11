//! Gishki Vanity
use super::prelude::*;

pub const ID: u32 = 93506862;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GISHKI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // act limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.chainop(e,tp,eg,ep,ev,re,r,rp)`
fn chainop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and rc:IsSetCard(SET_GISHKI) and rc:IsType(TYPE_RITUAL) then
    if effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && card::is_set_card(d, rc, &[SET_GISHKI as u16]) && card::is_type(d, rc, TYPE_RITUAL) {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
/// `s.sucfilter(c)`
fn sucfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GISHKI) and c:IsType(TYPE_RITUAL) and c:IsRitualSummoned()
    card::is_set_card(d, c, &[SET_GISHKI as u16]) && card::is_type(d, c, TYPE_RITUAL) && card::is_ritual_summoned(d, c)
}
/// `s.sucop(e,tp,eg,ep,ev,re,r,rp)`
fn sucop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.sucfilter,1,nil) then e:SetLabel(1)
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| sucfilter(d, c), 1, NO_EX) {
        effect::set_label(d, a.e, &[1]);
    } else {
        // else e:SetLabel(0) end
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.cedop(e,tp,eg,ep,ev,re,r,rp)`
fn cedop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS) and e:GetLabelObject():GetLabel()==1 then
    if duel::check_event(d, EVENT_SPSUMMON_SUCCESS).is_some() {
        let le = effect::get_label_object(d, a.e).as_effect().nil();
        if effect::get_label(d, le) == 1 {
            // Duel.SetChainLimitTillChainEnd(s.chainlm)
            duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
        }
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetOperation(s.chainop)
    effect::set_operation(d, e1, Some(operation(chainop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.sucop)
    effect::set_operation(d, e2, Some(operation(sucop)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e3, EVENT_CHAIN_END);
    // e3:SetOperation(s.cedop)
    effect::set_operation(d, e3, Some(operation(cedop)));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
