//! Xyz Territory
use super::prelude::*;

pub const ID: u32 = 4545854;

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
    // ad up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(lua_fn(adcon)));
    // e2:SetTarget(s.adtg)
    effect::set_target(d, e2, Some(effect_card_fn(adtg)));
    // e2:SetValue(s.adval)
    effect::set_value_fn(d, e2, effect_card_value(adval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy replace
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}

/// `s.adcon(e)`
fn adcon(d: &mut Duel, _a: Args) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and Duel.GetAttackTarget()
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()
}

/// `s.adtg(e,c)`
fn adtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d);
    // return (c==a or c==d) and c:IsType(TYPE_XYZ)
    (Some(c) == a || Some(c) == dd) && card::is_type(d, c, TYPE_XYZ)
}

/// `s.adval(e,c)`
fn adval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetRank()*200
    card::get_rank(d, c) as i64 * 200
}

/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not e:GetHandler():IsReason(REASON_REPLACE+REASON_RULE)
    //   and Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE + REASON_RULE)
            && duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_EFFECT, None);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_EFFECT)
        duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_EFFECT, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
