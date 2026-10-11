//! アモルファスP
//! Amorphous Persona
use super::prelude::*;

pub const ID: u32 = 23160024;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[98287529], listed_series: &[SET_AMORPHAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_AMORPHAGE))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, cc| card::is_set_card(d, cc, &[SET_AMORPHAGE as u16]))));
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // draw
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCode(EVENT_RELEASE)
    effect::set_code(d, e4, EVENT_RELEASE);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(2)
    effect::set_count_limit(d, e4, 2, 0, 0, 0);
    // e4:SetCondition(s.drcon)
    effect::set_condition(d, e4, Some(condition(drcon)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Ritual Summon
    // local e5=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,filter=aux.FilterBoolFunction(Card.IsCode,98287529),lv=8,matfilter=aux.FilterBoolFunction(Card.IsType,TYPE_PENDULUM)})
    let e5 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_code(d, x, &[98287529]))),
            lv: Some(ritual::Lv::N(8)),
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8| card::is_type(d, x, TYPE_PENDULUM)))),
            ..Default::default()
        },
    );
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e5, LOCATION_GRAVE);
    // e5:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e5, Some(cost::self_banish()));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousSetCard(SET_AMORPHAGE) and c:IsReason(REASON_RELEASE) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::is_previous_set_card(d, c, &[SET_AMORPHAGE as u16])
        && card::is_reason(d, c, REASON_RELEASE)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, cc| cfilter(d, cc, a.tp), 1, NO_EX)
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,e:GetHandler():GetCode())
    let h = effect::get_handler(d, a.e).nil();
    let code = card::get_code(d, h)[0];
    duel::hint(d, HINT_CARD as u8, 0, code as u64);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
