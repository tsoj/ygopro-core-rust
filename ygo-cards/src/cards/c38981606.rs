//! Desert Protector
//! 砂漠の守護者
use super::prelude::*;

pub const ID: u32 = 38981606;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // defup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_fn(value));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsSpellTrap,0,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)*300
    duel::get_matching_group_count(d, Some(&|d, c| card::is_spell_trap(d, c)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64 * 300
}
/// `s.dfilter(c,tp)`
fn dfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsRace(RACE_INSECT)
    //     and not c:IsReason(REASON_REPLACE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_race(d, c, RACE_INSECT as u64)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_controler(d, c, tp)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not eg:IsContains(e:GetHandler())
    //     and eg:IsExists(s.dfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let h = effect::get_handler(d, a.e).nil();
        return !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| dfilter(d, c, tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    //     return true
    // else return false end
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
/// `s.value(e,c)`
fn value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsRace(RACE_INSECT)
    //     and not c:IsReason(REASON_REPLACE) and c:IsControler(e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    dfilter(d, c, p)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT|REASON_REPLACE)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT | REASON_REPLACE, None, None);
}
