//! Everlasting Alloy
use super::prelude::*;

pub const ID: u32 = 37042505;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={CARD_JINZO}
    listed_names: &[CARD_JINZO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your machine monsters cannot be destroyed by opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition1)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.operation1)
    effect::set_operation(d, e1, Some(operation(operation1)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate card/effect that targets your machine monster(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetOperation(s.operation2)
    effect::set_operation(d, e2, Some(operation(operation2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition1(e,tp)
fn condition1(d: &mut Duel, tp: u8) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_JINZO),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[CARD_JINZO]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    condition1(d, a.tp)
}
// function s.target1(e,tp,eg,ep,ev,re,r,rp,chk)
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_MACHINE),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_MACHINE))), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
// function s.operation1(e,tp,eg,ep,ev,re,r,rp)
fn operation1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_MACHINE),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_MACHINE))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3060)
        effect::set_description(d, e1, 3060);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(s.efilter)
        effect::set_value_fn(d, e1, lua_fn(efilter));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// function s.efilter(e,re,rp)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandlerPlayer()==1-rp
    effect::get_handler_player(d, a.e(0)) == 1 - a.p(2)
}
// function s.tgfilter(c,tp)
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsMonster()
    card::is_controler(d, c, tp) && card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_monster(d, c)
}
// function s.condition2(e,tp,eg,ep,ev,re,r,rp)
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    // if not s.condition1(e,tp) or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if !condition1(d, a.tp) || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).remove(0);
    // return tg and tg:IsExists(s.tgfilter,1,nil,tp) and Duel.IsChainDisablable(ev)
    match tg.as_group().cloned() {
        Some(tg) => group::is_exists(d, &tg, &|d, x| tgfilter(d, x, a.tp), 1, NO_EX) && duel::is_chain_disablable(d, a.ev as u8),
        None => false,
    }
}
// function s.operation2(e,tp,eg,ep,ev,re,r,rp)
fn operation2(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
