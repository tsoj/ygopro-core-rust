//! Mokey Mokey Smackdown
use super::prelude::*;

pub const ID: u32 = 1965724;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[27288416], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Register if a face-up Fairy monster you control is destroyed while you control "Mokey Mokey"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(function(e) return e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1) end)
    effect::set_operation(
        d,
        e1,
        Some(operation(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        })),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // The ATK of each "Mokey Mokey" you control will become 3000
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, c, ID, None)
    })));
    // e2:SetTarget(function(e,c) return c:IsCode(27288416) and c:IsFaceup() end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[27288416]) && card::is_faceup(d, c))));
    // e2:SetValue(3000)
    effect::set_value(d, e2, 3000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.regconfilter(c,tp)
fn regconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousRaceOnField(RACE_FAIRY) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //   and c:IsPreviousControler(tp)
    card::is_previous_race_on_field(d, c, RACE_FAIRY)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
}
// function s.regcon(e,tp,eg,ep,ev,re,r,rp)
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():HasFlagEffect(id) and eg:IsExists(s.regconfilter,1,nil,tp)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,27288416),tp,LOCATION_MZONE,0,1,nil)
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    if card::has_flag_effect(d, c, ID, None) {
        return false;
    }
    if !group::is_exists(d, &eg, &|d, x| regconfilter(d, x, tp), 1, NO_EX) {
        return false;
    }
    let f = aux::faceup_filter(|d, x| card::is_code(d, x, &[27288416]));
    duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
