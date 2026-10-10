//! Needle Wall
use super::prelude::*;

pub const ID: u32 = 38299233;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // roll and destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetCondition(s.rdcon)
    effect::set_condition(d, e2, Some(condition(rdcon)));
    // e2:SetTarget(s.rdtg)
    effect::set_target(d, e2, Some(cost_target(rdtg)));
    // e2:SetOperation(s.rdop)
    effect::set_operation(d, e2, Some(operation(rdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rdcon(e,tp,...)`
fn rdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.rdtg(e,tp,...,chk)`
fn rdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
/// `s.rdop(e,tp,...)`
fn rdop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local d1=6
    let mut d1: i32 = 6;
    // while d1==6 do
    while d1 == 6 {
        // d1=Duel.TossDice(tp,1)
        d1 = duel::toss_dice(d, a.tp, 1, 0)[0];
    }
    // local tc=Duel.GetFieldCard(1-tp,LOCATION_MZONE,d1-1)
    let tc = duel::get_field_card(d, 1 - a.tp, LOCATION_MZONE, (d1 - 1) as u32);
    // if tc then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = tc {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
