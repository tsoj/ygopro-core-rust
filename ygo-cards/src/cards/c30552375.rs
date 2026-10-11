//! Grid Rod
use super::prelude::*;

pub const ID: u32 = 30552375;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // aux.AddEquipProcedure(c,0,aux.FilterBoolFunction(Card.IsRace,RACE_CYBERSE))
    aux::add_equip_procedure(d, c, Some(0), Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_CYBERSE))), None, None, None, None, None, None);
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // immune
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_COUNT);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetValue(s.indct)
    effect::set_value_fn(d, e3, lua_fn(indct));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to grave
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCondition(s.indcon)
    effect::set_condition(d, e4, Some(condition(indcon)));
    // e4:SetTarget(s.indtg)
    effect::set_target(d, e4, Some(cost_target(indtg)));
    // e4:SetOperation(s.indop)
    effect::set_operation(d, e4, Some(operation(indop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandlerPlayer()~=re:GetOwnerPlayer()
    let e = a.e(0);
    let re = a.e(1);
    effect::get_handler_player(d, e) != effect::get_owner_player(d, re)
}
/// `s.indct(e,re,r,rp)`
fn indct(_d: &mut Duel, a: Args) -> bool {
    // return r&REASON_BATTLE~=0
    (a.int(2) & REASON_BATTLE as i64) != 0
}
/// `s.indcon(e,tp,eg,ep,ev,re,r,rp)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsLocation(LOCATION_GRAVE)
    card::is_previous_position(d, c, POS_FACEUP) && card::is_location(d, c, LOCATION_GRAVE)
}
/// `s.indtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn indtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_CYBERSE),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.indop(e,tp,eg,ep,ev,re,r,rp)`
fn indop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_CYBERSE),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
