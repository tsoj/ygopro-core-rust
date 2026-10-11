//! Toon Buster Blader
use super::prelude::*;

pub const ID: u32 = 61190918;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15259703], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.atklimit)
    effect::set_operation(d, e1, Some(operation(atklimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Increase ATK
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetValue(s.val)
    effect::set_value_fn(d, e4, effect_card_value(val));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --Direct attack
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e5, EFFECT_DIRECT_ATTACK);
    // e5:SetCondition(s.dircon)
    effect::set_condition(d, e5, Some(condition(dircon)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atklimit(e,tp,eg,ep,ev,re,r,rp)`
fn atklimit(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.dirfilter1(c)`
fn dirfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(15259703)
    card::is_faceup(d, c) && card::is_code(d, c, &[15259703])
}
/// `s.dirfilter2(c)`
fn dirfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TOON)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.dirfilter1,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    //   and not Duel.IsExistingMatchingCard(s.dirfilter2,e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&dirfilter1), p, LOCATION_ONFIELD, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&dirfilter2), p, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    card::is_race(d, c, RACE_DRAGON) && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),0,LOCATION_GRAVE|LOCATION_MZONE,nil)*500
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, 0, LOCATION_GRAVE | LOCATION_MZONE, NO_EX) as i64 * 500
}
