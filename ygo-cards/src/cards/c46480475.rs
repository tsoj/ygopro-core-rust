//! Discord
use super::prelude::*;

pub const ID: u32 = 46480475;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn splimit(_d: &mut Duel, a: Args) -> bool {
    // return (sumtp&SUMMON_TYPE_SYNCHRO)==SUMMON_TYPE_SYNCHRO
    let sumtp = a.int(3) as u64;
    (sumtp & SUMMON_TYPE_SYNCHRO as u64) == SUMMON_TYPE_SYNCHRO as u64
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetLabel(3)
    effect::set_label(d, e1, &[3]);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_SELF_TURN,3)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 3);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
    true
}
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    // ct=ct-1
    let ct = effect::get_label(d, a.e) - 1;
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    // if ct==0 then
    if ct == 0 {
        // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
        let h = effect::get_handler(d, a.e).nil();
        duel::send_to_grave(d, h, REASON_EFFECT, None, None);
    }
}
