//! Pyramid of Wonders
use super::prelude::*;

pub const ID: u32 = 66835946;

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
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_ZOMBIE))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_ZOMBIE))));
    // e2:SetValue(s.val)
    effect::set_value_fn(d, e2, effect_card_value(val));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy replace
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.val(e,c)
fn val(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_MZONE)*200
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) as i64 * 200
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local tc=eg:GetFirst()
        let eg = a.eg.clone().nil();
        let tc = group::get_first(d, &eg);
        // return #eg==1 and tc:IsLocation(LOCATION_MZONE) and tc:IsControler(tp) and tc:IsFaceup() and tc:IsRace(RACE_ZOMBIE)
        //     and not tc:IsReason(REASON_REPLACE)
        return eg.len() == 1 && {
            let tc = tc.nil();
            card::is_location(d, tc, LOCATION_MZONE)
                && card::is_controler(d, tc, tp)
                && card::is_faceup(d, tc)
                && card::is_race(d, tc, RACE_ZOMBIE)
                && !card::is_reason(d, tc, REASON_REPLACE)
        };
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_EFFECT, None, None);
}
