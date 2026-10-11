//! Alien Infiltrator
use super::prelude::*;

pub const ID: u32 = 76573247;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(aux.seqmovcon)
    effect::set_condition(d, e1, Some(aux::seqmovcon()));
    // e1:SetTarget(aux.seqmovtg)
    effect::set_target(d, e1, Some(aux::seqmovtg()));
    // e1:SetOperation(aux.seqmovtgop)
    effect::set_operation(d, e1, Some(aux::seqmovtgop()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // direct attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // e2:SetCondition(s.dircon)
    effect::set_condition(d, e2, Some(condition(dircon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetColumnGroup():FilterCount(Card.IsControler,nil,1-e:GetHandlerPlayer())==0
    let h = effect::get_handler(d, a.e).nil();
    let column = card::get_column_group(d, h, 0, 0);
    let p = 1 - effect::get_handler_player(d, a.e);
    group::filter_count(d, &column, &|d, c| card::is_controler(d, c, p), NO_EX) == 0
}
