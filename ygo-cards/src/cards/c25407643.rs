//! Secret Sanctuary of the Spellcasters
use super::prelude::*;

pub const ID: u32 = 25407643;

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
    // Destroy itself if you control no spellcaster monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e2, EFFECT_SELF_DESTROY);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // When a monster is normal summoned to opponent's field, it cannot attack or activate its effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Same as above, but when special summoned
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.descon(e)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_SPELLCASTER)), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.cfilter1(c)`
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSpell()
    card::is_faceup(d, c) && card::is_spell(d, c)
}
/// `s.cfilter2(c,tp)`
fn cfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and not c:IsRace(RACE_SPELLCASTER) and c:IsControler(tp)
    card::is_faceup(d, c) && !card::is_race(d, c, RACE_SPELLCASTER) && card::is_controler(d, c, tp)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_SZONE,0,1,e:GetHandler())
    //     and not Duel.IsExistingMatchingCard(s.cfilter1,tp,0,LOCATION_SZONE,1,nil)
    //     and eg:IsExists(s.cfilter2,1,nil,1-tp)
    let h = effect::get_handler(d, a.e).nil();
    if !duel::is_existing_matching_card(d, Some(&cfilter1), a.tp, LOCATION_SZONE, 0, 1, h.into()) {
        return false;
    }
    if duel::is_existing_matching_card(d, Some(&cfilter1), a.tp, 0, LOCATION_SZONE, 1, NO_EX) {
        return false;
    }
    let eg = a.eg.clone().nil();
    let tp = 1 - a.tp;
    group::is_exists(d, &eg, &|d, x| cfilter2(d, x, tp), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg.clone().nil());
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if not Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_SZONE,0,1,e:GetHandler())
    //     or Duel.IsExistingMatchingCard(s.cfilter1,tp,0,LOCATION_SZONE,1,nil) then return end
    if !duel::is_existing_matching_card(d, Some(&cfilter1), a.tp, LOCATION_SZONE, 0, 1, c.into()) || duel::is_existing_matching_card(d, Some(&cfilter1), a.tp, 0, LOCATION_SZONE, 1, NO_EX) {
        return;
    }
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    let tp = a.tp;
    for tc in group::iter(d, &eg) {
        // if tc:IsRelateToEffect(e) and s.cfilter2(tc,1-tp) then
        if card::is_relate_to_effect(d, tc, a.e) && cfilter2(d, tc, 1 - tp) {
            // Cannot attack
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // Cannot activate its effects
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(3302)
            effect::set_description(d, e2, 3302);
            // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
