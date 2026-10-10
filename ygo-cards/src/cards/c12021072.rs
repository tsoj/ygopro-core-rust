//! Child's Play
use super::prelude::*;

pub const ID: u32 = 12021072;

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
    // heal
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.hcondition)
    effect::set_condition(d, e2, Some(condition(hcondition)));
    // e2:SetOperation(s.hoperation)
    effect::set_operation(d, e2, Some(operation(hoperation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // e3:SetCondition(s.hcondition2)
    effect::set_condition(d, e3, Some(condition(hcondition2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // battle protection
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_BATTLE);
    // e5:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e5, LOCATION_MZONE, 0);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCondition(s.bpcondition)
    effect::set_condition(d, e5, Some(condition(bpcondition)));
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.hcondition(e,tp,eg,...)`
fn hcondition(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    match &a.eg {
        Some(eg) => group::is_exists(d, eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX),
        None => false,
    }
}
/// `s.hcondition2(e,tp,eg,...)`
fn hcondition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(Card.IsControler,1,nil,1-tp)
    match &a.eg {
        Some(eg) => group::is_exists(d, eg, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX),
        None => false,
    }
}
/// `s.hoperation(e,tp)`
fn hoperation(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Recover(tp,300,REASON_EFFECT)
    duel::recover(d, a.tp, 300, REASON_EFFECT, false, None);
}
/// `s.bpcondition(e)`
fn bpcondition(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(e:GetHandlerPlayer())>=10000
    let p = effect::get_handler_player(d, a.e);
    duel::get_lp(d, p) >= 10000
}
