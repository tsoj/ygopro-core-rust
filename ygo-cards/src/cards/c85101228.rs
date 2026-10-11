//! Morphtronic Bind
use super::prelude::*;

pub const ID: u32 = 85101228;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_MORPHTRONIC}
    listed_series: &[SET_MORPHTRONIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // cannot attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::get_level(d, c) >= 4)));
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // pos limit
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
    effect::set_code(d, e3, EFFECT_CANNOT_CHANGE_POSITION);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.con(e)
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MORPHTRONIC),e:GetHandler():GetControler(),LOCATION_MZONE,0,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]))), p, LOCATION_MZONE, 0, 1, NO_EX)
}
